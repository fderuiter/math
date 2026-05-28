extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};
use syn::visit_mut::VisitMut;

struct InjectorVisitor;

impl VisitMut for InjectorVisitor {
    fn visit_expr_mut(&mut self, node: &mut syn::Expr) {
        syn::visit_mut::visit_expr_mut(self, node);
        
        match node {
            syn::Expr::Binary(expr) => {
                use syn::BinOp::*;
                match expr.op {
                    Add(_) | Sub(_) | Mul(_) | Div(_) | Rem(_) |
                    BitXor(_) | BitAnd(_) | BitOr(_) | Shl(_) | Shr(_) => {
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

#[proc_macro_attribute]
pub fn verified(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);
    
    // Count statements in the top-level block
    let statement_count = input_fn.block.stmts.len();
    
    if statement_count > 60 {
        return syn::Error::new_spanned(
            &input_fn.sig.ident,
            format!("Function exceeds 60 statements (NASA Power of 10 Rule 4) - length: {} statements", statement_count)
        ).to_compile_error().into();
    }
    
    let fn_block = &input_fn.block;
    let new_block: syn::Block = syn::parse_quote! {
        {
            verified_engine::metrics::increment_calls();
            #fn_block
        }
    };
    
    *input_fn.block = new_block;
    
    let mut visitor = InjectorVisitor;
    visitor.visit_item_fn_mut(&mut input_fn);

    TokenStream::from(quote! { #input_fn })
}
