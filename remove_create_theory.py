import re
import glob

# Remove fn create_theory() { ... } from UnifiedModel impls
pattern = re.compile(r'\s*fn create_theory.*?\{.*?\n    \}', re.DOTALL)

for path in glob.glob('math_explorer_gui/src/**/*.rs', recursive=True):
    with open(path, 'r') as f:
        content = f.read()
    
    new_content = pattern.sub('', content)
    if new_content != content:
        with open(path, 'w') as f:
            f.write(new_content)
