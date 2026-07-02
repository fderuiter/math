import re

with open('math_explorer_gui/src/tabs/morphogenesis.rs', 'r') as f:
    content = f.read()

content = re.sub(r'<<<<<<< HEAD\n.*?\n=======\n.*?\n>>>>>>> origin/main\n?', '', content, flags=re.DOTALL)

with open('math_explorer_gui/src/tabs/morphogenesis.rs', 'w') as f:
    f.write(content)
