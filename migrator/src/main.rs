use std::fs;
use std::path::PathBuf;
use toml_edit::{Document, Item, value, Table};
use walkdir::WalkDir;

fn main() {
    let mut all_deps = std::collections::BTreeMap::new();

    // Pass 1: Collect external dependencies and find max versions
    for entry in WalkDir::new("/app").into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.ends_with("Cargo.toml") && !path.to_string_lossy().contains("/target/") && !path.to_string_lossy().contains("/egui_plot/") && !path.to_string_lossy().contains("/migrator/") {
            if path.to_string_lossy() == "/app/Cargo.toml" {
                continue;
            }
            let content = fs::read_to_string(path).unwrap();
            let doc = content.parse::<Document>().unwrap();
            
            let dep_sections = vec!["dependencies", "dev-dependencies", "build-dependencies"];
            
            for section in &dep_sections {
                if let Some(deps) = doc.get(section).and_then(|i| i.as_table_like()) {
                    for (k, v) in deps.iter() {
                        if k == "egui_plot" || (v.is_table_like() && v.as_table_like().unwrap().contains_key("path")) {
                            continue;
                        }
                        if !all_deps.contains_key(&k.to_string()) {
                            all_deps.insert(k.to_string(), v.clone());
                        } else {
                            let existing = all_deps.get(&k.to_string()).unwrap();
                            if v.to_string().len() > existing.to_string().len() {
                                all_deps.insert(k.to_string(), v.clone());
                            }
                        }
                    }
                }
            }
            
            if let Some(target) = doc.get("target").and_then(|i| i.as_table_like()) {
                for (_, target_table) in target.iter() {
                    if let Some(deps) = target_table.as_table_like().unwrap().get("dependencies").and_then(|i| i.as_table_like()) {
                        for (k, v) in deps.iter() {
                            if k == "egui_plot" || (v.is_table_like() && v.as_table_like().unwrap().contains_key("path")) {
                                continue;
                            }
                            if !all_deps.contains_key(&k.to_string()) {
                                all_deps.insert(k.to_string(), v.clone());
                            } else {
                                let existing = all_deps.get(&k.to_string()).unwrap();
                                if v.to_string().len() > existing.to_string().len() {
                                    all_deps.insert(k.to_string(), v.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some(v) = all_deps.get_mut("eframe") {
        *v = value("0.33.3");
    }

    // Pass 2: Write into workspace Cargo.toml
    let root_cargo_path = PathBuf::from("/app/Cargo.toml");
    let mut root_doc = fs::read_to_string(&root_cargo_path).unwrap().parse::<Document>().unwrap();
    
    if root_doc.get("workspace").is_none() {
        root_doc["workspace"] = Item::Table(Table::new());
    }
    if root_doc["workspace"].get("dependencies").is_none() {
        root_doc["workspace"]["dependencies"] = Item::Table(Table::new());
    }
    
    let ws_deps = root_doc["workspace"]["dependencies"].as_table_mut().unwrap();
    for (k, v) in &all_deps {
        let mut val = v.clone();
        if val.is_inline_table() {
            let mut inline = val.as_inline_table().unwrap().clone();
            inline.remove("optional");
            val = Item::Value(toml_edit::Value::InlineTable(inline));
        }
        ws_deps.insert(k, val);
    }
    fs::write(&root_cargo_path, root_doc.to_string()).unwrap();
}
