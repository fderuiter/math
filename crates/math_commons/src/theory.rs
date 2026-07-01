use std::collections::HashMap;

/// A structured representation of mathematical constraints for a parameter.
#[derive(Debug, Clone)]
pub struct ParameterConstraint {
    pub min: f64,
    pub max: f64,
    pub step: f64,
}

/// A standardized trait for exporting human-readable, verified descriptions
/// for every mathematical model.
pub trait TheoryDescribable {
    /// Returns the verified human-readable description for accessibility.
    fn theory_description(&self) -> String;

    /// Returns the citation for the mathematical model.
    fn theory_citation(&self) -> String;

    /// Returns a list of all available descriptions for a given mathematical state or operation.
    fn available_descriptions(&self) -> HashMap<String, String>;

    /// Returns theoretical constraints for parameter fields of the model.
    fn theory_parameters(&self) -> HashMap<String, ParameterConstraint> {
        HashMap::new()
    }
}

/// A macro active within the GUI crate context to fetch the theoretical description.
#[macro_export]
macro_rules! theory_verification {
    ($obj:expr) => {
        $obj.theory_description()
    };
}
