use eframe::egui;

#[cfg(feature = "ai")]
pub mod ai;
#[cfg(feature = "pure_math")]
pub mod analysis;
#[cfg(feature = "applied")]
pub mod battery_degradation;
#[cfg(feature = "physics")]
pub mod chaos;
#[cfg(feature = "climate")]
pub mod climate;
#[cfg(feature = "applied")]
pub mod clinical_trials;
#[cfg(feature = "epidemiology")]
pub mod epidemiology;
#[cfg(feature = "applied")]
pub mod favoritism;
#[cfg(feature = "pure_math")]
pub mod financial_math;
#[cfg(feature = "physics")]
pub mod fluid_dynamics;
#[cfg(feature = "applied")]
pub mod game_theory;
#[cfg(feature = "pure_math")]
pub mod geometry_topology;
#[cfg(feature = "pure_math")]
pub mod graph_theory;
#[cfg(feature = "physics")]
pub mod medical;
#[cfg(feature = "biology")]
pub mod morphogenesis;
#[cfg(feature = "physics")]
pub mod mri;
#[cfg(feature = "biology")]
pub mod neuroscience;
#[cfg(feature = "pure_math")]
pub mod number_theory;
#[cfg(feature = "physics")]
pub mod quantum;
#[cfg(feature = "physics")]
pub mod solid_state;
pub mod traceability;
#[cfg(feature = "strict-opt-in-experimental")]
pub mod experimental_tab;

#[cfg(feature = "pure_math")]
pub use geometry_topology::GeometryTopologyTab;
pub use traceability::TraceabilityTab;

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

// [cite:graph_parameters_rust]
