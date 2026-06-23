import os, glob, re, sys, argparse, fnmatch

def main():
    parser = argparse.ArgumentParser(description="Traceability validation script")
    parser.add_argument('--exclude', action='append', default=[], help='Patterns to exclude')
    args = parser.parse_args()

    paper_files = glob.glob('papers/*.tex')
    papers = set(os.path.basename(f).replace('.tex', '') for f in paper_files)

    rs_files = []
    for root, dirs, files in os.walk('.'):
        # Default exclusions for non-source directories
        dirs[:] = [d for d in dirs if d not in ('.git', 'target', 'build', 'tmp')]
        
        for file in files:
            if file.endswith('.rs'):
                path = os.path.normpath(os.path.join(root, file))
                # Normalize path separators
                path = path.replace('\\', '/')
                rs_files.append(path)

    # Filter out excluded files/directories based on --exclude flags
    filtered_rs_files = []
    for f in rs_files:
        excluded = False
        for ex in args.exclude:
            # Match anywhere in the path, or exact filename match
            if fnmatch.fnmatch(f, ex) or fnmatch.fnmatch(f, f"*/{ex}/*") or fnmatch.fnmatch(f, f"{ex}/*") or fnmatch.fnmatch(f, f"*/{ex}") or ex in f:
                excluded = True
                break
        if not excluded:
            filtered_rs_files.append(f)

    cite_regex = re.compile(r'\[cite:([a-zA-Z0-9_.-]+)\]')

    cited_papers = set()
    invalid_cites = False
    scanned_count = 0

    print("=== Traceability Report ===")
    print("Files Scanned:")

    for f in filtered_rs_files:
        print(f" - {f}")
        scanned_count += 1
        with open(f, 'r', encoding='utf-8') as file:
            content = file.read()
        matches = cite_regex.findall(content)
        for m in matches:
            cited_papers.add(m)
            if m not in papers:
                print(f"   [!] INVALID citation: {m}")
                invalid_cites = True

    print("\n=== Validation Failures ===")
    
    orphans = papers - cited_papers
    if invalid_cites:
        print("Invalid citations were found (see details above).")
    if orphans:
        print(f"Orphaned papers found: {orphans}")
        
    print(f"\nSummary: Scanned {scanned_count} source files.")
    
    if invalid_cites or orphans:
        sys.exit(1)
    print("All checks passed!")

if __name__ == "__main__":
    main()
