use eframe::egui::{Response, WidgetText};
use scientific_metadata::theory::TheoryDescribable;

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
#[allow(missing_docs)]
pub fn init_accessibility_bridge() {}

#[cfg(target_arch = "wasm32")]
// theory_verification!
pub fn announce_status(message: &str) {
    announce_status_with_priority(message, "polite");
}

#[cfg(target_arch = "wasm32")]
pub fn announce_status_with_priority(message: &str, priority: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(el) = document.get_element_by_id("aria-live-region") {
                let _ = el.set_attribute("aria-live", priority);
                if el.text_content().as_deref() != Some(message) {
                    el.set_text_content(Some(message));
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(missing_docs)]
pub fn announce_status(_message: &str) {}

#[cfg(not(target_arch = "wasm32"))]
#[allow(missing_docs)]
pub fn announce_status_with_priority(_message: &str, _priority: &str) {}

#[allow(missing_docs)]
pub trait AccessibleHoverText {
    #[allow(missing_docs)]
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

#[allow(missing_docs)]
pub trait AccessibleTheoryHover {
    #[allow(missing_docs)]
    fn accessible_theory_hover(self, theory: &impl TheoryDescribable) -> Self;
}

impl AccessibleTheoryHover for Response {
    fn accessible_theory_hover(self, theory: &impl TheoryDescribable) -> Self {
        let raw_text = theory.theory_description();
        let speech_text = theory.phonetic_description();

        if self.hovered() || self.has_focus() {
            announce_status(&speech_text);
        }

        self.on_hover_text(raw_text)
    }
}
// theory_verification!
