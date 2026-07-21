import sys

# Example input line from machete:
# crate_name -- ./path/to/Cargo.toml:
# \tdep1
# \tdep2

current_file = None
unused_deps = {}

with open(sys.argv[1], 'r') as f:
    for line in f:
        line = line.rstrip('\n')
        if not line:
            continue
        if line.endswith('Cargo.toml:'):
            current_file = line.split('--')[-1].strip()
            # remove colon
            if current_file.endswith(':'):
                current_file = current_file[:-1]
            unused_deps[current_file] = []
        elif line.startswith('\t') and current_file:
            dep = line.strip()
            unused_deps[current_file].append(dep)

# now iterate over files and modify
for path, deps_to_remove in unused_deps.items():
    if not deps_to_remove:
        continue
        
    print(f"Modifying {path} to remove {deps_to_remove}")
    with open(path, 'r') as f:
        lines = f.readlines()
        
    out_lines = []
    # very simple removal, but dependencies could span multiple lines if it's not inline table
    # however, we changed them all to `{ workspace = true }` so they are on a single line!
    for line in lines:
        keep = True
        for d in deps_to_remove:
            if line.strip().startswith(f"{d} ="):
                keep = False
                break
        if keep:
            out_lines.append(line)
            
    with open(path, 'w') as f:
        f.writelines(out_lines)
