import re

with open('math_explorer_gui/src/tabs/morphogenesis.rs', 'r') as f:
    content = f.read()

# Fix conflict 1
content = re.sub(r'<<<<<<< HEAD\n(.*?)\n=======\n.*?\n>>>>>>> origin/main', r'\1', content, flags=re.DOTALL)

with open('math_explorer_gui/src/tabs/morphogenesis.rs', 'w') as f:
    f.write(content)
