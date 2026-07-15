import json, subprocess, os
def run_check():
    env = os.environ.copy()
    env["RUSTUP_TOOLCHAIN"] = "stable"
    result = subprocess.run(["cargo", "check", "--workspace", "--message-format=json"], capture_output=True, text=True, env=env)
    return result.stdout
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
print(f"Found {len(warnings)} missing_docs warnings")
file_edits = {}
for w in warnings:
    spans = [s for s in w.get("spans", []) if s.get("is_primary")]
    if not spans: continue
    span = spans[0]
    file_name = span["file_name"]
    line_start = span["line_start"]
    msg_text = w["message"]
    if file_name not in file_edits: file_edits[file_name] = []
    file_edits[file_name].append({"line": line_start, "msg": msg_text})
for file_name, edits in file_edits.items():
    print(f"File {file_name}: {len(edits)} edits")
