use crate::accessibility::AccessibleHoverText;
use eframe::egui;
use math_commons::theory::TheoryDescribable;

/// Renders a UI parameter directly from the theoretical constraints.
pub fn render_theory_parameter<T: TheoryDescribable>(
    ui: &mut egui::Ui,
    model: &T,
    param_name: &str,
    label: &str,
    value: &mut f64,
) -> egui::Response {
    let params = model.theory_parameters();
    let constraint = params
        .get(param_name)
        .unwrap_or_else(|| panic!("Parameter '{}' not found in theory metadata", param_name));

    let slider = egui::Slider::new(value, constraint.min..=constraint.max)
        .step_by(constraint.step)
        .text(label);

    let response = ui.add(slider);

    let tooltip = format!(
        "{}\n\nCitation: {}",
        model.theory_description(),
        model.theory_citation()
    );

    response.accessible_hover_text(tooltip)
}

/// Fallback mechanism for complex parameters that require a custom UI layout
/// but still need to be validated against theoretical limits.
pub fn get_theory_constraint<T: TheoryDescribable>(
    model: &T,
    param_name: &str,
) -> math_commons::theory::ParameterConstraint {
    model
        .theory_parameters()
        .get(param_name)
        .cloned()
        .unwrap_or_else(|| panic!("Parameter '{}' not found in theory metadata", param_name))
}
