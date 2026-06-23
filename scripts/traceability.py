import os, glob, re, sys

paper_files = glob.glob('papers/*.tex')
papers = set(os.path.basename(f).replace('.tex', '') for f in paper_files)

source_files = []
text_exts = {'.rs', '.md', '.txt', '.toml', '.py', '.sh', '.c', '.cpp', '.h'}
exclude_dirs = {'target', '.git', 'node_modules', '__pycache__'}

for root, dirs, files in os.walk('.'):
    dirs[:] = [d for d in dirs if d not in exclude_dirs]
    for f in files:
        if any(f.endswith(ext) for ext in text_exts):
            source_files.append(os.path.join(root, f))

cite_regex = re.compile(r'\[cite:([a-zA-Z0-9_.-]+)\]')
macro_regex = re.compile(r'theory_verification!\s*\([^)]*?paper\s*=\s*"([^"]+)"')

cited_papers = set()
invalid_cites = False

for f in source_files:
    try:
        with open(f, 'r', encoding='utf-8') as file:
            content = file.read()
    except Exception:
        continue
        
    matches = cite_regex.findall(content)
    macro_matches = macro_regex.findall(content)
    macro_matches = [m.replace('.tex', '') for m in macro_matches]
    
    all_matches = matches + macro_matches
    
    for m in all_matches:
        cited_papers.add(m)
        if m not in papers:
            print(f"File {f} cites INVALID paper {m}!")
            invalid_cites = True

orphans = papers - cited_papers
if orphans:
    print(f"Orphaned papers found: {orphans}")
    
if invalid_cites or orphans:
    sys.exit(1)
print("All checks passed!")
