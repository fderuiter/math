use std::env;
use std::fs;
use std::path::Path;

#[allow(clippy::too_many_lines)]
fn main() {
    println!("cargo:rerun-if-changed=../../papers");
    println!("cargo:rerun-if-changed=../../math_explorer/src");
    println!("cargo:rerun-if-changed=../../math_explorer_gui/src/tabs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("vfs_data.rs");

    let mut map_entries = String::new();
    let mut dirs_map = String::new();

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
",
        map_entries, dirs_map
    );

    fs::write(dest_path, code).unwrap();
}
