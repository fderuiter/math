#[macro_export]
macro_rules! theory_verification {
    (
        module = $module_name:ident,
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

            // Import the constants from the generated VFS module.
            // If the paper is missing or the constant is not defined in the paper,
            // this will fail at compile time, satisfying the theory integrity requirement.
            use oxidize_core::vfs::vfs_data::theory_constants::$module_name;

            $(
                #[allow(dead_code)]
                const $const_name: f64 = $const_val;
            )*

            #[test]
            #[verified_engine::verified]
            fn test_theory_verification() {
                // Check that the module is registered in the shared engine registry
                if !oxidize_core::traceability::TraceabilityEngine::verify_module_registered(stringify!($module_name)) {
                    panic!("Traceability mismatch: module '{}' is not registered to any paper", stringify!($module_name));
                }

                // Enforce parity between manually provided constants and the paper's constants
                $(
                    assert_relative_eq!(
                        $const_name,
                        oxidize_core::vfs::vfs_data::theory_constants::$module_name::$const_name,
                        epsilon = $epsilon,
                        max_relative = $epsilon
                    );
                )*

                $test_body
            }
        }
    };
}
