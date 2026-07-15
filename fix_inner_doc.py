import os

for root, _, files in os.walk("."):
    if ".git" in root or "target" in root: continue
    for f in files:
        if f.endswith(".rs"):
            path = os.path.join(root, f)
            with open(path, "r") as file:
                lines = file.readlines()
            
            needs_fix = False
            for i in range(min(5, len(lines)-1)):
                if lines[i].strip() == "#[allow(missing_docs)]" and lines[i+1].strip() == "//! Legacy crate.":
                    lines[i], lines[i+1] = lines[i+1], lines[i]
                    needs_fix = True
                
            if needs_fix:
                with open(path, "w") as file:
                    file.writelines(lines)
