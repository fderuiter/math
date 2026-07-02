import re
import os

files = [
    'math_explorer_gui/src/tabs/fluid_dynamics/lattice_boltzmann.rs',
    'math_explorer_gui/src/tabs/quantum/wave_sim.rs'
]

def extract_head(text):
    # This just strips out the ======= to >>>>>>> part.
    # But wait, PR 1051 added `TheoryDescribable` impls for the old tool structures in origin/main!
    # I should instead add `TheoryDescribable` for the new `UnifiedModel` structures.
    # Let's write a targeted replace.
    pattern = re.compile(r'<<<<<<< HEAD\n(.*?)=======\n.*?\n>>>>>>> origin/main\n', re.DOTALL)
    return pattern.sub(r'\1', text)

for file in files:
    with open(file, 'r') as f:
        content = f.read()
    content = extract_head(content)
    with open(file, 'w') as f:
        f.write(content)
