#[macro_export]
macro_rules! theory_verification {
    (
        module = $module_name:expr,
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
            #[verified_engine::verified]
            fn test_theory_verification() {
                // Check that the module is registered in the shared engine registry
                if !oxidize_core::traceability::TraceabilityEngine::verify_module_registered($module_name) {
                    panic!("Traceability mismatch: module '{}' is not registered to any paper", $module_name);
                }

                $test_body
            }
        }
    };
}
