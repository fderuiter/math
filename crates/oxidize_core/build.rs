use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashSet;
use regex::Regex;

#[allow(clippy::too_many_lines)]
fn main() {
    println!("cargo:rerun-if-changed=../../papers");
    println!("cargo:rerun-if-changed=../../math_explorer/src");
    println!("cargo:rerun-if-changed=../../math_explorer_gui/src/tabs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("vfs_data.rs");

    let mut map_entries = String::new();
    let mut dirs_map = String::new();
    let mut theory_constants_map = String::new();

    let num_regex = r"([+-]?(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:[eE][+-]?[0-9]+)?)";
    let ident = r"([A-Za-z_][A-Za-z0-9_]*)";
    
    let p1 = format!(r"\\newcommand\{{\\{}\}}\{{{}\}}", ident, num_regex);
    let p2 = format!(r"\\def\\{}\s*\{{{}\}}", ident, num_regex);
    let p3 = format!(r"{}\s*=\s*{}", ident, num_regex);
    let pattern = format!("{}|{}|{}", p1, p2, p3);
    
    let re = Regex::new(&pattern).unwrap();

    let root_dirs = vec![
        "../../papers",
        "../../math_explorer/src",
        "../../math_explorer_gui/src/tabs",
    ];

    for root in root_dirs {
        let root_path = Path::new(root);
        if !root_path.exists() {
            continue;
        }

        let mut stack = vec![root_path.to_path_buf()];
        while let Some(current) = stack.pop() {
            let relative_dir = current
                .strip_prefix("../../")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            if let Ok(entries) = fs::read_dir(&current) {
                let mut children = Vec::new();
                for entry in entries.flatten() {
                    let path = entry.path();
                    let name = entry.file_name().to_str().unwrap().to_string();
                    children.push(name.clone());

                    let relative_path = path
                        .strip_prefix("../../")
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();

                    if path.is_file() {
                        if relative_path.ends_with(".tex") || relative_path.ends_with(".rs") {
                            let content = fs::read_to_string(&path).unwrap_or_default();
                            map_entries.push_str(&format!(
                                "{:?} => Some({:?}),\n",
                                relative_path, content
                            ));
                            
                            // If it's a .tex file, extract constants
                            if relative_path.ends_with(".tex") {
                                let module_name = path.file_stem().unwrap().to_str().unwrap();
                                let mut constants_code = String::new();
                                let mut seen = HashSet::new();

                                for cap in re.captures_iter(&content) {
                                    let (name, val) = if let Some(n) = cap.get(1) {
                                        (n.as_str(), cap.get(2).unwrap().as_str())
                                    } else if let Some(n) = cap.get(3) {
                                        (n.as_str(), cap.get(4).unwrap().as_str())
                                    } else if let Some(n) = cap.get(5) {
                                        (n.as_str(), cap.get(6).unwrap().as_str())
                                    } else {
                                        continue;
                                    };
                                    
                                    if seen.insert(name.to_string()) {
                                        let val_str = if val.contains('.') || val.contains('e') || val.contains('E') {
                                            val.to_string()
                                        } else {
                                            format!("{}.0", val)
                                        };
                                        constants_code.push_str(&format!("            pub const {}: f64 = {};\n", name, val_str));
                                    }
                                }
                                theory_constants_map.push_str(&format!("
        pub mod {} {{
{}
        }}\n", module_name, constants_code));
                            }
                        } else {
                            map_entries.push_str(&format!("{:?} => Some(\"\"),\n", relative_path));
                        }
                    } else if path.is_dir() {
                        stack.push(path);
                    }
                }
                let child_str = children
                    .iter()
                    .map(|c| format!("{:?}", c))
                    .collect::<Vec<_>>()
                    .join(", ");
                dirs_map.push_str(&format!("{:?} => Some(&[{}]),\n", relative_dir, child_str));
            }
        }
    }

    let code = format!(
        "
#[allow(clippy::too_many_lines)]
pub fn get_file_content(path: &str) -> Option<&'static str> {{
    match path {{
        {}
        _ => None,
    }}
}}

pub fn get_dir_children(path: &str) -> Option<&'static [&'static str]> {{
    match path {{
        {}
        _ => None,
    }}
}}

pub mod theory_constants {{
    #![allow(non_upper_case_globals, clippy::too_many_lines)]
{}
}}
",
        map_entries, dirs_map, theory_constants_map
    );

    fs::write(dest_path, code).unwrap();
}
