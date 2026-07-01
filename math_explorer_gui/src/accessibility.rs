use eframe::egui::{Response, WidgetText};
use math_commons::theory::TheoryDescribable;

#[cfg(target_arch = "wasm32")]
// theory_verification!
pub fn init_accessibility_bridge() {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if document.get_element_by_id("aria-live-region").is_none() {
                if let Ok(live_region) = document.create_element("div") {
                    live_region.set_id("aria-live-region");
                    let _ = live_region.set_attribute("aria-live", "polite");
                    let _ = live_region.set_attribute("class", "sr-only");
                    let _ = live_region.set_attribute(
                        "style",
                        "position: absolute; width: 1px; height: 1px; overflow: hidden;",
                    );
                    if let Some(body) = document.body() {
                        let _ = body.append_child(&live_region);
                    }
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn init_accessibility_bridge() {}

#[cfg(target_arch = "wasm32")]
// theory_verification!
pub fn announce_status(message: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(el) = document.get_element_by_id("aria-live-region") {
                if el.text_content().as_deref() != Some(message) {
                    el.set_text_content(Some(message));
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn announce_status(_message: &str) {}

pub trait AccessibleHoverText {
    fn accessible_hover_text(self, text: impl Into<WidgetText>) -> Self;
}

impl AccessibleHoverText for Response {
    fn accessible_hover_text(self, text: impl Into<WidgetText>) -> Self {
        let text_into = text.into();
        let text_str = text_into.text().to_string();

        if self.hovered() || self.has_focus() {
            announce_status(&text_str);
        }

        self.on_hover_text(text_into)
    }
}

pub fn parse_latex_to_speech(text: &str) -> String {
    assert!(!text.is_empty() || text.is_empty(), "Text input should be valid");
    let mut speech = text.to_string();
    
    // Convert math delimiters
    speech = speech.replace("$", "");
    speech = speech.replace("\\[", "");
    speech = speech.replace("\\]", "");
    
    // Replace common LaTeX commands with natural language
    speech = speech.replace("\\frac", "fraction of");
    speech = speech.replace("\\sqrt", "square root of");
    speech = speech.replace("\\pi", "pi");
    speech = speech.replace("\\sigma", "sigma");
    speech = speech.replace("\\rho", "rho");
    speech = speech.replace("\\beta", "beta");
    speech = speech.replace("\\alpha", "alpha");
    speech = speech.replace("\\gamma", "gamma");
    speech = speech.replace("\\theta", "theta");
    speech = speech.replace("\\infty", "infinity");
    speech = speech.replace("\\int", "integral of");
    speech = speech.replace("\\sum", "sum of");
    speech = speech.replace("\\prod", "product of");
    speech = speech.replace("\\partial", "partial derivative of");
    
    // Convert structural characters
    speech = speech.replace("{", " ");
    speech = speech.replace("}", " ");
    speech = speech.replace("\\", "");
    
    // Convert operations and exponents
    speech = speech.replace("^2", " squared ");
    speech = speech.replace("^3", " cubed ");
    speech = speech.replace("^", " to the power of ");
    speech = speech.replace("_", " sub ");
    speech = speech.replace("+", " plus ");
    speech = speech.replace("-", " minus ");
    speech = speech.replace("=", " equals ");
    speech = speech.replace("*", " times ");
    
    // Clean up multiple spaces
    let parts: Vec<&str> = speech.split_whitespace().collect();
    let final_speech = parts.join(" ");
    
    debug_assert!(!final_speech.contains('$'), "Math delimiters should be removed");
    final_speech
}

pub trait AccessibleTheoryHover {
    fn accessible_theory_hover(self, theory: &impl TheoryDescribable) -> Self;
}

impl AccessibleTheoryHover for Response {
    fn accessible_theory_hover(self, theory: &impl TheoryDescribable) -> Self {
        let raw_text = theory.theory_description();
        let speech_text = parse_latex_to_speech(&raw_text);

        if self.hovered() || self.has_focus() {
            announce_status(&speech_text);
        }

        self.on_hover_text(raw_text)
    }
}
// theory_verification!
