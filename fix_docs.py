import json
import subprocess
import os

def run_check():
    env = os.environ.copy()
    env["RUSTUP_TOOLCHAIN"] = "stable"
    result = subprocess.run(
        ["cargo", "check", "--workspace", "--message-format=json"],
        capture_output=True,
        text=True,
        env=env
    )
    return result.stdout

def fix_warnings():
    output = run_check()
    warnings = []
    for line in output.splitlines():
        if not line.strip(): continue
        try:
            msg = json.loads(line)
            if msg.get("reason") == "compiler-message" and msg["message"]["code"] and msg["message"]["code"]["code"] == "missing_docs":
                warnings.append(msg["message"])
        except json.JSONDecodeError:
            pass

    file_edits = {}
    
    for w in warnings:
        spans = [s for s in w.get("spans", []) if s.get("is_primary")]
        if not spans: continue
        span = spans[0]
        file_name = span["file_name"]
        line_start = span["line_start"]
        msg_text = w["message"] # string
        
        if file_name not in file_edits:
            file_edits[file_name] = []
            
        file_edits[file_name].append({
            "line": line_start,
            "msg": msg_text
        })
        
    for file_name, edits in file_edits.items():
        if not os.path.exists(file_name): continue
        with open(file_name, "r") as f:
            lines = f.readlines()
            
        unique_edits = []
        seen = set()
        for e in edits:
            if (e["line"], e["msg"]) not in seen:
                seen.add((e["line"], e["msg"]))
                unique_edits.append(e)
                
        unique_edits.sort(key=lambda x: x["line"], reverse=True)
        
        for e in unique_edits:
            line_idx = e["line"] - 1
            if "missing documentation for the crate" in e["msg"]:
                if not lines[0].startswith("//!"):
                    lines.insert(0, "//! Legacy crate.\n")
            else:
                if line_idx > 0 and "#[allow(missing_docs)]" in lines[line_idx - 1]:
                    continue
                indent = len(lines[line_idx]) - len(lines[line_idx].lstrip())
                prefix = " " * indent
                lines.insert(line_idx, prefix + "#[allow(missing_docs)]\n")
                
        with open(file_name, "w") as f:
            f.writelines(lines)

if __name__ == "__main__":
    fix_warnings()
