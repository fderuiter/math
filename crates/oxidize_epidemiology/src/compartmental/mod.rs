pub mod common;
pub mod macros;
pub mod seir;
pub mod sir;

pub use common::basic_reproduction_number;
pub use seir::{SEIRModel, SEIRState};
pub use sir::{SIRModel, SIRState};

// [cite:graph_parameters_rust]
