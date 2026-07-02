import re
with open('math_explorer_gui/src/async_sim/unified.rs', 'r') as f:
    content = f.read()

# Fix conflict 1
conflict1 = """<<<<<<< HEAD

    /// Return the theoretical context.
    fn create_theory() -> Option<Box<dyn math_commons::theory::TheoryDescribable>>
    where
        Self: Sized,
    {
        None
    }
=======
>>>>>>> origin/main"""
resolved1 = ""
content = content.replace(conflict1, resolved1)

# Fix conflict 2
conflict2 = """<<<<<<< HEAD
                    if let Some(theory) = &self.theory_instance {
                        resp = resp.accessible_hover_text(theory.theory_description());
=======
                    
                    if let Some(desc) = available_descs.get(name) {
                        resp = resp.accessible_hover_text(desc);
>>>>>>> origin/main"""
resolved2 = """                    if let Some(desc) = available_descs.get(name) {
                        resp = resp.accessible_hover_text(desc);"""
content = content.replace(conflict2, resolved2)

with open('math_explorer_gui/src/async_sim/unified.rs', 'w') as f:
    f.write(content)
