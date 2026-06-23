import os, glob, re, sys

paper_files = glob.glob('papers/*.tex')
papers = set(os.path.basename(f).replace('.tex', '') for f in paper_files)

rs_files = glob.glob('math_explorer/src/**/mod.rs', recursive=True) + \
           glob.glob('crates/**/mod.rs', recursive=True) + \
           glob.glob('math_explorer_gui/src/tabs/**/*.rs', recursive=True)

cite_regex = re.compile(r'\[cite:([a-zA-Z0-9_.-]+)\]')

cited_papers = set()
invalid_cites = False
missing_cites = False

for f in rs_files:
    with open(f, 'r') as file:
        content = file.read()
    matches = cite_regex.findall(content)
    if not matches:
        print(f"File {f} is missing a citation!")
        missing_cites = True
    else:
        for m in matches:
            cited_papers.add(m)
            if m not in papers:
                print(f"File {f} cites INVALID paper {m}!")
                invalid_cites = True

orphans = papers - cited_papers
if orphans:
    print(f"Orphaned papers found: {orphans}")
    
if missing_cites or invalid_cites or orphans:
    sys.exit(1)
print("All checks passed!")
