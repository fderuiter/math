use eframe::egui;

pub mod ai;
pub mod battery_degradation;
pub mod chaos;
pub mod clinical_trials;
pub mod epidemiology;
pub mod favoritism;
pub mod financial_math;
pub mod fluid_dynamics;
pub mod game_theory;
pub mod medical;
pub mod morphogenesis;
pub mod mri;
pub mod neuroscience;
pub mod number_theory;
pub mod quantum;
pub mod solid_state;

/// A trait for defining a tab in the Math Explorer application.
///
/// This trait serves as the primary extension point for the GUI. Each mathematical domain
/// (e.g., MRI Physics, Game Theory) should implement this trait as a standalone struct.
///
/// # Example
///
/// ```rust
/// use math_explorer_gui::tabs::ExplorerTab;
/// use eframe::egui;
///
/// struct MyTab {
///     counter: i32,
/// }
///
/// impl ExplorerTab for MyTab {
///     fn name(&self) -> &'static str {
///         "My Tab"
///     }
///
///     fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
///         egui::CentralPanel::default().show(ctx, |ui| {
///             ui.heading("My Tab");
///             if ui.button("Increment").clicked() {
///                 self.counter += 1;
///             }
///             ui.label(format!("Count: {}", self.counter));
///         });
///     }
/// }
/// ```
pub trait ExplorerTab {
    /// Returns the name of the tab, displayed in the navigation bar.
    fn name(&self) -> &'static str;

    /// Renders the content of the tab.
    ///
    /// The tab is responsible for defining its own panels (e.g., `SidePanel`, `CentralPanel`).
    /// The parent application will typically have already drawn a top navigation bar, so avoid `TopBottomPanel::top`.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The egui Context, used for adding widgets and handling input.
    /// * `frame` - The eframe Frame, used for window management (e.g., resizing, closing).
    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
}
