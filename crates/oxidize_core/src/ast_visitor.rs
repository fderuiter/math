use std::collections::HashMap;
use syn::visit::{self, Visit};
use syn::{ItemFn, ItemMacro, Macro, Meta};

pub struct AstVisitor {
    pub verified_modules: Vec<String>,
    pub module_tiers: HashMap<String, String>,
    pub has_vacuous_bypass: bool,
    pub total_funcs: usize,
    pub total_asserts: usize,
    pub verified_funcs: usize,
    pub verified_asserts: usize,
    pub active_submodules: Vec<String>,
    pub opted_out: bool,
}

impl Default for AstVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl AstVisitor {
    pub fn new() -> Self {
        Self {
            verified_modules: Vec::new(),
            module_tiers: HashMap::new(),
            has_vacuous_bypass: false,
            total_funcs: 0,
            total_asserts: 0,
            verified_funcs: 0,
            verified_asserts: 0,
            active_submodules: Vec::new(),
            opted_out: false,
        }
    }
}

impl<'ast> Visit<'ast> for AstVisitor {
    fn visit_file(&mut self, node: &'ast syn::File) {
        for attr in &node.attrs {
            if let Meta::List(list) = &attr.meta {
                let path_str = quote::quote!(#list).to_string().replace(" ", "");
                if path_str.contains("opt_out") {
                    self.opted_out = true;
                }
            }
        }
        visit::visit_file(self, node);
    }

    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        if node.content.is_none() {
            self.active_submodules.push(node.ident.to_string());
        }
        visit::visit_item_mod(self, node);
    }

    fn visit_item_macro(&mut self, node: &'ast ItemMacro) {
        if let Some(ident) = node.mac.path.segments.last().map(|s| &s.ident) {
            let name = ident.to_string();
            let tier = if name == "theory_verification" {
                Some("Deterministic")
            } else if name == "stochastic_signature_verification" {
                Some("Stochastic")
            } else if name == "empirical_verification" {
                Some("Empirical")
            } else {
                None
            };

            if let Some(tier_name) = tier {
                let tokens = node.mac.tokens.to_string();

                // Detect vacuous bypass: zero initializations in stochastic/empirical
                if tokens.contains("zeros(")
                    || tokens.contains("zeros_like")
                    || tokens.contains("0.0")
                    || tokens.contains("fill(0)")
                {
                    self.has_vacuous_bypass = true;
                }

                if let Some(idx) = tokens.find("module = \"") {
                    let start = idx + 10;
                    if let Some(end) = tokens[start..].find('"') {
                        let module_name = &tokens[start..start + end];
                        self.verified_modules.push(module_name.to_string());
                        self.module_tiers
                            .insert(module_name.to_string(), tier_name.to_string());
                    }
                }
            }
        }
        visit::visit_item_macro(self, node);
    }

    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        self.total_funcs += 1;

        let mut is_verified = false;
        for attr in &node.attrs {
            if let Meta::Path(path) = &attr.meta {
                let path_str = quote::quote!(#path).to_string().replace(" ", "");
                if path_str == "verified_engine::verified" || path_str == "verified" {
                    is_verified = true;
                    break;
                }
            } else if let Meta::List(list) = &attr.meta {
                let path_str = quote::quote!(#list).to_string().replace(" ", "");
                if path_str.starts_with("verified_engine::verified")
                    || path_str.starts_with("verified")
                {
                    is_verified = true;
                    break;
                }
            }
        }

        if is_verified {
            self.verified_funcs += 1;
        }

        // Count assertions in this function
        let mut assert_visitor = AssertVisitor { count: 0 };
        visit::visit_item_fn(&mut assert_visitor, node);

        self.total_asserts += assert_visitor.count;
        if is_verified {
            self.verified_asserts += assert_visitor.count;
        }

        // Continue visiting inside the function (if there are nested items)
        visit::visit_item_fn(self, node);
    }
}

struct AssertVisitor {
    count: usize,
}

impl<'ast> Visit<'ast> for AssertVisitor {
    fn visit_macro(&mut self, node: &'ast Macro) {
        if let Some(ident) = node.path.segments.last().map(|s| &s.ident) {
            let name = ident.to_string();
            if name == "assert"
                || name == "assert_eq"
                || name == "assert_ne"
                || name == "debug_assert"
                || name == "debug_assert_eq"
                || name == "debug_assert_ne"
            {
                self.count += 1;
            }
        }
        visit::visit_macro(self, node);
    }
}
