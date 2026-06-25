#[macro_export]
macro_rules! theory_verification {
    (
        module = $module_name:expr,
        paper = $paper_name:expr,
        epsilon = $epsilon:expr,
        constants = {
            $( $const_name:ident = $const_val:expr; )*
        },
        test = $test_body:block
    ) => {
        #[cfg(test)]
        #[allow(unused_imports, clippy::all)]
        mod theory_verification {
            use super::*;
            use approx::assert_relative_eq;

            $(
                #[allow(dead_code)]
                const $const_name: f64 = $const_val;
            )*

            #[test]
            fn test_theory_verification() {
                // Cross-reference module name with paper name using shared engine
                if !oxidize_core::traceability::TraceabilityEngine::check_naming_parity($module_name, $paper_name) {
                    println!("cargo:warning=Naming parity mismatch: module '{}' maps to paper '{}'", $module_name, $paper_name);
                }

                $test_body
            }
        }
    };
}
