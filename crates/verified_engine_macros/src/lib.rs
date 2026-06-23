extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::{ItemFn, parse_macro_input};

struct InjectorVisitor;

impl VisitMut for InjectorVisitor {
    fn visit_expr_mut(&mut self, node: &mut syn::Expr) {
        syn::visit_mut::visit_expr_mut(self, node);

        match node {
            syn::Expr::Binary(expr) => {
                use syn::BinOp::*;
                match expr.op {
                    Add(_) | Sub(_) | Mul(_) | Div(_) | Rem(_) | BitXor(_) | BitAnd(_)
                    | BitOr(_) | Shl(_) | Shr(_) => {
                        let original = expr.clone();
                        *node = syn::parse_quote!({
                            verified_engine::metrics::increment_arithmetic();
                            #original
                        });
                    }
                    _ => {}
                }
            }
            syn::Expr::Index(expr) => {
                let original = expr.clone();
                *node = syn::parse_quote!({
                    verified_engine::metrics::increment_memory_loads();
                    #original
                });
            }
            _ => {}
        }
    }
}

struct DeepAstVisitor {
    statements: usize,
    assertions: usize,
    direct_recursion: bool,
    function_name: String,
}

impl<'ast> Visit<'ast> for DeepAstVisitor {
    fn visit_stmt(&mut self, node: &'ast syn::Stmt) {
        self.statements += 1;
        syn::visit::visit_stmt(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        if let Some(ident) = node.path.segments.last().map(|s| s.ident.to_string()) {
            if ident == "assert"
                || ident == "assert_eq"
                || ident == "assert_ne"
                || ident == "debug_assert"
            {
                self.assertions += 1;
            }
        }
        syn::visit::visit_macro(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if let syn::Expr::Path(ref expr_path) = *node.func {
            if let Some(ident) = expr_path.path.segments.last().map(|s| s.ident.to_string()) {
                if ident == self.function_name {
                    self.direct_recursion = true;
                }
            }
        }
        syn::visit::visit_expr_call(self, node);
    }
}

#[proc_macro_attribute]
pub fn verified(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);
    let attr_str = attr.to_string();

    let is_opt_out = attr_str.contains("opt_out");

    let mut visitor = DeepAstVisitor {
        statements: 0,
        assertions: 0,
        direct_recursion: false,
        function_name: input_fn.sig.ident.to_string(),
    };

    visitor.visit_item_fn(&input_fn);

    if !is_opt_out {
        if visitor.direct_recursion {
            return syn::Error::new_spanned(
                &input_fn.sig.ident,
                "Direct recursion is not allowed in high-integrity verified modules (NASA Power of 10 Rule 1). Use #[verified(opt_out = \"reason\")] to bypass.",
            ).to_compile_error().into();
        }

        if visitor.statements > 60 {
            return syn::Error::new_spanned(
                &input_fn.sig.ident,
                format!("Function exceeds 60 statements (NASA Power of 10 Rule 4) - length: {} statements", visitor.statements)
            ).to_compile_error().into();
        }

        if visitor.assertions < 2 {
            return syn::Error::new_spanned(
                &input_fn.sig.ident,
                format!("Assertion density is below 2.0 (NASA Power of 10 Rule 5) - found: {} assertions, required: 2", visitor.assertions)
            ).to_compile_error().into();
        }
    }

    let fn_block = &input_fn.block;
    let new_block: syn::Block = syn::parse_quote! {
        {
            verified_engine::metrics::increment_calls();
            #fn_block
        }
    };

    *input_fn.block = new_block;

    let mut injector = InjectorVisitor;
    injector.visit_item_fn_mut(&mut input_fn);

    TokenStream::from(quote! { #input_fn })
}
