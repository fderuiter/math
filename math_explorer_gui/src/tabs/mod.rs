
#[cfg(feature = "biology")]
pub mod biology;
use eframe::egui;

#[cfg(feature = "domain_logic")]
pub mod ai;
#[cfg(feature = "domain_logic")]
pub mod analysis;
#[cfg(feature = "domain_logic")]
pub mod battery_degradation;
#[cfg(feature = "domain_logic")]
pub mod chaos;
#[cfg(feature = "domain_logic")]
pub mod climate;
#[cfg(feature = "domain_logic")]
pub mod clinical_trials;
#[cfg(feature = "domain_logic")]
pub mod epidemiology;
#[cfg(feature = "domain_logic")]
pub mod favoritism;
#[cfg(feature = "domain_logic")]
pub mod financial_math;
#[cfg(feature = "domain_logic")]
pub mod fluid_dynamics;
#[cfg(feature = "domain_logic")]
pub mod game_theory;
#[cfg(feature = "domain_logic")]
pub mod geometry_topology;
#[cfg(feature = "domain_logic")]
pub mod graph_theory;
#[cfg(feature = "domain_logic")]
pub mod medical;
#[cfg(feature = "domain_logic")]

#[cfg(feature = "domain_logic")]
pub mod mri;
#[cfg(feature = "domain_logic")]

#[cfg(feature = "domain_logic")]
pub mod number_theory;
#[cfg(feature = "domain_logic")]
pub mod quantum;
#[cfg(feature = "domain_logic")]
pub mod solid_state;

#[cfg(feature = "domain_logic")]
pub use geometry_topology::GeometryTopologyTab;

/// A trait for defining a tab in the Math Explorer application.
///
/// This trait serves as the primary extension point for the GUI. Each mathematical domain
/// (e.g., MRI Physics, Game Theory) should implement this trait as a standalone struct.
///
/// # Example
///
/// ```rust,no_run
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
