pub mod citation_registry;
pub mod constants;
pub mod diagnostics;
pub mod error;
pub mod math_kernel;
pub mod theory;

pub mod generated_schemas {
    include!(concat!(env!("OUT_DIR"), "/generated_schemas.rs"));
}
