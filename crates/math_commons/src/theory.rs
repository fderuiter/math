use std::collections::HashMap;

/// A standardized trait for exporting human-readable, verified descriptions 
/// for every mathematical model.
pub trait TheoryDescribable {
    /// Returns the verified human-readable description for accessibility.
    fn theory_description(&self) -> String;
    
    /// Returns the citation for the mathematical model.
    fn theory_citation(&self) -> String;
    
    /// Returns a list of all available descriptions for a given mathematical state or operation.
    fn available_descriptions(&self) -> HashMap<String, String>;
}

/// A macro active within the GUI crate context to fetch the theoretical description.
#[macro_export]
macro_rules! theory_verification {
    ($obj:expr) => {
        $obj.theory_description()
    };
}
