import os
import re

ws_deps = []
with open('/app/Cargo.toml', 'r') as f:
    in_ws = False
    for line in f:
        line = line.strip()
        if line == "[workspace.dependencies]":
            in_ws = True
        elif in_ws and line.startswith("["):
            in_ws = False
        elif in_ws and "=" in line:
            name = line.split("=")[0].strip()
            ws_deps.append(name)

for root, dirs, files in os.walk('/app'):
    if 'target' in root or 'egui_plot' in root or 'migrator' in root:
        continue
    if 'Cargo.toml' in files and root != '/app':
        path = os.path.join(root, 'Cargo.toml')
        with open(path, 'r') as f:
            content = f.read()
        
        lines = content.split('\n')
        out = []
        for line in lines:
            if '=' in line and not line.strip().startswith('#'):
                parts = line.split('=', 1)
                name = parts[0].strip()
                val = parts[1].strip()
                if name in ws_deps:
                    if "workspace = true" not in val and "path =" not in val:
                        # Case 1: val is just a string version e.g. "1.0.0"
                        if val.startswith('"') and val.endswith('"'):
                            out.append(f"{parts[0].split(name)[0]}{name} = {{ workspace = true }}")
                            continue
                        # Case 2: val is an inline table
                        if val.startswith('{') and val.endswith('}'):
                            # Replace `version = "..."` with `workspace = true`
                            new_val = re.sub(r'version\s*=\s*"[^"]*"', 'workspace = true', val)
                            # Remove default-features = false if present (because it's only allowed in workspace.dependencies)
                            new_val = re.sub(r',\s*default-features\s*=\s*false', '', new_val)
                            new_val = re.sub(r'default-features\s*=\s*false\s*,', '', new_val)
                            out.append(f"{parts[0].split(name)[0]}{name} = {new_val}")
                            continue
            out.append(line)
        
        with open(path, 'w') as f:
            f.write('\n'.join(out))
