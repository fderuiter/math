import os
import re

deps = {}

def process_file(path):
    with open(path, 'r') as f:
        content = f.read()

    # Find [dependencies], [dev-dependencies], [target.*.dependencies] blocks
    blocks = re.split(r'(\[(?:dev-)?dependencies\]|\[target\..*?\.dependencies\])', content)
    if len(blocks) == 1:
        return

    out = [blocks[0]]
    for i in range(1, len(blocks), 2):
        header = blocks[i]
        body = blocks[i+1]
        out_body = []
        
        for line in body.split('\n'):
            match = re.match(r'^([a-zA-Z0-9_-]+)\s*=\s*(.*)', line.strip())
            if match:
                name = match.group(1)
                val = match.group(2)
                
                # Exclude path dependencies and vendored egui_plot
                if name == "egui_plot" or 'path =' in val:
                    out_body.append(line)
                else:
                    if name not in deps:
                        deps[name] = val
                    else:
                        # Simple logic: if new val is longer (more features), or higher version, we should merge.
                        # For now, just print to resolve conflicts manually.
                        if deps[name] != val:
                            print(f"Conflict for {name}: {deps[name]} vs {val}")
                            # Pick the more complex one, or one with features
                            if "features" in val and "features" not in deps[name]:
                                deps[name] = val
                            elif len(val) > len(deps[name]) and "features" in val:
                                deps[name] = val
                            elif "0.33.3" in val: # example to pick newer
                                deps[name] = val
                            elif "0.8.6" in val:
                                deps[name] = val
                            elif "2.0.18" in val:
                                deps[name] = val
                            elif "1.0.228" in val:
                                deps[name] = val
                            elif "2.0.118" in val:
                                deps[name] = val
                    
                    if line.startswith(" "): # keep indentation if any
                        out_body.append(f"{name} = {{ workspace = true }}")
                    else:
                        out_body.append(f"{name} = {{ workspace = true }}")
            else:
                out_body.append(line)
        out.append(header)
        out.append('\n'.join(out_body))
        
    with open(path, 'w') as f:
        f.write(''.join(out))

for root, dirs, files in os.walk('/app'):
    if '/target/' in root or '/egui_plot' in root or '/.git' in root:
        continue
    if 'Cargo.toml' in files:
        if root != '/app':
            process_file(os.path.join(root, 'Cargo.toml'))

print("WORKSPACE DEPS:")
for k, v in sorted(deps.items()):
    print(f"{k} = {v}")
