use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::{Parse, ParseStream}, LitStr, Ident, Token};
use std::path::PathBuf;
use std::fs;
use std::hash::{Hash, Hasher};

pub struct EmbedTheoryArgs {
    pub file_path: String,
    pub labels: Vec<String>,
}

impl Parse for EmbedTheoryArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let file_path_lit: LitStr = input.parse()?;
        let file_path = file_path_lit.value();
        
        let mut labels = Vec::new();
        
        while !input.is_empty() {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
            
            let ident: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            
            if ident == "label" {
                let label_lit: LitStr = input.parse()?;
                labels.push(label_lit.value());
            } else if ident == "labels" {
                // To keep it simple, just expect multiple label="..." arguments if needed
                // Or you can implement array parsing here
                return Err(syn::Error::new_spanned(ident, "Use multiple `label=\"...\"` arguments instead"));
            } else {
                return Err(syn::Error::new_spanned(ident, "Unknown attribute argument, expected `label`"));
            }
        }
        
        if labels.is_empty() {
            return Err(syn::Error::new(input.span(), "At least one `label=\"...\"` must be provided"));
        }
        
        Ok(EmbedTheoryArgs { file_path, labels })
    }
}

fn strip_labels(text: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = text.chars().collect();
    while i < chars.len() {
        if text[i..].starts_with("\\label{") {
            i += 7;
            let mut depth = 1;
            while i < chars.len() && depth > 0 {
                if chars[i] == '{' { depth += 1; }
                if chars[i] == '}' { depth -= 1; }
                i += 1;
            }
            continue;
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

fn convert_environment(env_name: &str, inner_content: &str) -> String {
    let inner = strip_labels(inner_content).trim().to_string();
    match env_name {
        "equation" | "equation*" | "align" | "align*" | "eqnarray" | "eqnarray*" | "displaymath" => {
            format!("$$\n{}\n$$", inner)
        }
        "lemma" | "theorem" | "proof" | "definition" | "corollary" | "proposition" => {
            let title = env_name[0..1].to_uppercase() + &env_name[1..];
            format!("**{}**\n\n{}", title, inner)
        }
        _ => {
            format!("{}\n", inner)
        }
    }
}

fn extract_label(tex_content: &str, target_label: &str) -> Option<String> {
    let label_str = format!("\\label{{{}}}", target_label);
    let label_pos = tex_content.find(&label_str)?;
    
    let before_label = &tex_content[..label_pos];
    
    if let Some(begin_pos) = before_label.rfind("\\begin{") {
        let env_start = &before_label[begin_pos..];
        if let Some(brace_end) = env_start.find('}') {
            let env_name = &env_start[7..brace_end];
            
            let end_str = format!("\\end{{{}}}", env_name);
            if let Some(end_pos) = tex_content[label_pos..].find(&end_str) {
                let inner_start = begin_pos + 7 + env_name.len() + 1;
                let inner_end = label_pos + end_pos;
                let inner = &tex_content[inner_start..inner_end];
                return Some(convert_environment(env_name, inner));
            }
        }
    }
    None
}

pub fn embed_theory_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = match syn::parse2::<EmbedTheoryArgs>(attr) {
        Ok(args) => args,
        Err(err) => return err.to_compile_error(),
    };
    
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    
    // Resolve absolute path
    // We assume the path given is relative to the workspace root
    let mut abs_path = PathBuf::from(&manifest_dir);
    // Since crates/verified_engine_macros is inside workspace, workspace root is ../../
    abs_path.push("../../");
    abs_path.push(&args.file_path);
    
    if !abs_path.exists() {
        // Try fallback to just relative to manifest
        let fallback = PathBuf::from(&manifest_dir).join(&args.file_path);
        if fallback.exists() {
            abs_path = fallback;
        } else {
            // Also try absolute if the user passed an absolute path
            let abs_direct = PathBuf::from(&args.file_path);
            if abs_direct.exists() {
                abs_path = abs_direct;
            } else {
                let err_msg = format!("LaTeX file not found: {}", args.file_path);
                return syn::Error::new(proc_macro2::Span::call_site(), err_msg).to_compile_error();
            }
        }
    }
    
    let canonical_path = abs_path.canonicalize().unwrap_or(abs_path.clone());
    let canonical_path_str = canonical_path.to_string_lossy().to_string();
    
    let tex_content = match fs::read_to_string(&canonical_path) {
        Ok(c) => c,
        Err(e) => {
            let err_msg = format!("Failed to read {}: {}", args.file_path, e);
            return syn::Error::new(proc_macro2::Span::call_site(), err_msg).to_compile_error();
        }
    };
    
    // Check for manual mathematical content in existing docstrings to prevent shadow-mirroring
    if let Ok(parsed_item) = syn::parse2::<syn::Item>(item.clone()) {
        let attrs = match &parsed_item {
            syn::Item::Fn(f) => &f.attrs,
            syn::Item::Struct(s) => &s.attrs,
            syn::Item::Enum(e) => &e.attrs,
            syn::Item::Mod(m) => &m.attrs,
            syn::Item::Impl(i) => &i.attrs,
            syn::Item::Trait(t) => &t.attrs,
            syn::Item::Type(t) => &t.attrs,
            syn::Item::Const(c) => &c.attrs,
            syn::Item::Static(s) => &s.attrs,
            _ => &[] as &[syn::Attribute],
        };
        for attr in attrs {
            if attr.path().is_ident("doc") {
                if let syn::Meta::NameValue(nv) = &attr.meta {
                    if let syn::Expr::Lit(expr_lit) = &nv.value {
                        if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                            let doc_val = lit_str.value();
                            if doc_val.contains("$$") || doc_val.contains("\\begin{equation}") || doc_val.contains("\\begin{align}") {
                                return syn::Error::new(
                                    attr.path().segments[0].ident.span(),
                                    "Manual docstring content for mathematical sections is strictly forbidden in modules using #[embed_theory] to prevent shadow-mirroring."
                                ).to_compile_error();
                            }
                        }
                    }
                }
            }
        }
    } else if let Ok(parsed_impl_item) = syn::parse2::<syn::ImplItem>(item.clone()) {
        let attrs = match &parsed_impl_item {
            syn::ImplItem::Fn(f) => &f.attrs,
            syn::ImplItem::Const(c) => &c.attrs,
            syn::ImplItem::Type(t) => &t.attrs,
            _ => &[] as &[syn::Attribute],
        };
        for attr in attrs {
            if attr.path().is_ident("doc") {
                if let syn::Meta::NameValue(nv) = &attr.meta {
                    if let syn::Expr::Lit(expr_lit) = &nv.value {
                        if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                            let doc_val = lit_str.value();
                            if doc_val.contains("$$") || doc_val.contains("\\begin{equation}") || doc_val.contains("\\begin{align}") {
                                return syn::Error::new(
                                    attr.path().segments[0].ident.span(),
                                    "Manual docstring content for mathematical sections is strictly forbidden in modules using #[embed_theory] to prevent shadow-mirroring."
                                ).to_compile_error();
                            }
                        }
                    }
                }
            }
        }
    }
    
    let mut docs = Vec::new();
    for label in &args.labels {
        if let Some(md) = extract_label(&tex_content, label) {
            docs.push(md);
        } else {
            let err_msg = format!("Label `{}` not found or not in a supported environment in {}", label, args.file_path);
            return syn::Error::new(proc_macro2::Span::call_site(), err_msg).to_compile_error();
        }
    }
    
    let doc_str = docs.join("\n\n");
    
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    canonical_path_str.hash(&mut hasher);
    for l in &args.labels {
        l.hash(&mut hasher);
    }
    let hash = hasher.finish();
    let track_ident = quote::format_ident!("_TRACK_THEORY_{:x}", hash);
    
    quote! {
        #[doc = #doc_str]
        #item
        
        #[allow(dead_code)]
        const #track_ident: &str = include_str!(#canonical_path_str);
    }
}
