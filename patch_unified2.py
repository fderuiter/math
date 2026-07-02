import re
with open('math_explorer_gui/src/async_sim/unified.rs', 'r') as f:
    content = f.read()

# Replace theory_instance field in UnifiedSimTool
struct_old = """    texture: Option<egui::TextureHandle>,
    theory_instance: Option<Box<dyn math_commons::theory::TheoryDescribable>>,
    _marker: std::marker::PhantomData<M>,"""
struct_new = """    texture: Option<egui::TextureHandle>,
    cached_theory_desc: String,
    cached_phonetic: String,
    cached_citation: String,
    cached_descs: HashMap<String, String>,
    _marker: std::marker::PhantomData<M>,"""
content = content.replace(struct_old, struct_new)

# Update new() implementation
new_old = """        let controller = SimulationController::new(runner);

        Self {
            controller,
            params,
            param_metadata,
            steps_per_frame: 5,
            last_snapshot: None,
            texture: None,
            theory_instance: M::create_theory(),
            _marker: std::marker::PhantomData,
        }"""
new_new = """        let controller = SimulationController::new(runner);

        let temp_model = M::new(&initial_params);

        Self {
            controller,
            params,
            param_metadata,
            steps_per_frame: 5,
            last_snapshot: None,
            texture: None,
            cached_theory_desc: temp_model.theory_description(),
            cached_phonetic: temp_model.phonetic_description(),
            cached_citation: temp_model.theory_citation(),
            cached_descs: temp_model.available_descriptions(),
            _marker: std::marker::PhantomData,
        }"""
content = content.replace(new_old, new_new)

# Update TheoryDescribable impl
theory_old = """impl<M: UnifiedModel> math_commons::theory::TheoryDescribable for UnifiedSimTool<M> {
    fn theory_description(&self) -> String { "Default theory description".into() }
    fn phonetic_description(&self) -> String { "Default phonetic description".into() }
    fn theory_citation(&self) -> String { "Default Citation, 2026".into() }
    fn available_descriptions(&self) -> std::collections::HashMap<String, String> { std::collections::HashMap::new() }
}"""
theory_new = """impl<M: UnifiedModel> math_commons::theory::TheoryDescribable for UnifiedSimTool<M> {
    fn theory_description(&self) -> String { self.cached_theory_desc.clone() }
    fn phonetic_description(&self) -> String { self.cached_phonetic.clone() }
    fn theory_citation(&self) -> String { self.cached_citation.clone() }
    fn available_descriptions(&self) -> std::collections::HashMap<String, String> { self.cached_descs.clone() }
}"""
content = content.replace(theory_old, theory_new)

# Remove old InteractiveTool theory method
old_theory_method = """    fn theory(&self) -> Option<&dyn math_commons::theory::TheoryDescribable> {
        self.theory_instance.as_deref()
    }"""
content = content.replace(old_theory_method, "")

with open('math_explorer_gui/src/async_sim/unified.rs', 'w') as f:
    f.write(content)
