use eframe::egui;

pub mod chaos;
pub mod epidemiology;
pub mod fluid_dynamics;
pub mod game_theory;
pub mod medical;
pub mod morphogenesis;
pub mod mri;
pub mod neuroscience;
pub mod quantum;
pub mod solid_state;

/// A trait for defining a tab in the Math Explorer application.
pub trait ExplorerTab {
    /// Returns the name of the tab.
    fn name(&self) -> &'static str;

    /// Renders the content of the tab.
    ///
    /// The tab is responsible for defining its own panels (e.g., SidePanel, CentralPanel).
    /// The parent application will typically have already drawn a top navigation bar.
    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
}
