import re

with open('math_explorer_gui/src/tabs/neuroscience/hodgkin_huxley.rs', 'r') as f:
    content = f.read()

def resolve_conflict(text):
    # Match <<<<<<< HEAD ... ======= ... >>>>>>> origin/main
    # keeping only the HEAD part
    pattern = re.compile(r'<<<<<<< HEAD\n(.*?)\n=======\n.*?\n>>>>>>> origin/main', re.DOTALL)
    return pattern.sub(r'\1', text)

content = resolve_conflict(content)

with open('math_explorer_gui/src/tabs/neuroscience/hodgkin_huxley.rs', 'w') as f:
    f.write(content)
