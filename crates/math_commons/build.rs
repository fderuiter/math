use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct Schema {
    id: String,
    name: String,
    description: String,
    citation: String,
    parameters: Vec<Parameter>,
}

#[derive(Deserialize)]
struct Parameter {
    id: String,
    label: String,
    #[serde(rename = "type")]
    type_name: String,
    default: f64,
    min: f64,
    max: f64,
    unit: String,
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root_dir = Path::new(&manifest_dir).parent().unwrap().parent().unwrap();
    let schemas_dir = root_dir.join("schemas");
    let papers_dir = root_dir.join("papers").join("tables");

    println!("cargo:rerun-if-changed={}", schemas_dir.display());

    let mut generated_rust = String::new();
    generated_rust.push_str("// AUTO-GENERATED CODE. DO NOT EDIT.\n");
    generated_rust.push_str("use crate::theory::TheoryDescribable;\n\n");

    let schemas = load_schemas(&schemas_dir);

    for schema in &schemas {
        generate_rust_struct(&mut generated_rust, schema);
        generate_latex_file(&papers_dir, schema);
    }

    generate_typed_model_command(&mut generated_rust, &schemas);

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_schemas.rs");
    fs::write(dest_path, generated_rust).unwrap();
}

fn load_schemas(schemas_dir: &Path) -> Vec<Schema> {
    let mut schemas = Vec::new();
    if let Ok(entries) = fs::read_dir(schemas_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path).unwrap();
                let schema: Schema = serde_json::from_str(&content).unwrap();
                schemas.push(schema);
            }
        }
    }
    schemas
}

fn generate_rust_struct(generated_rust: &mut String, schema: &Schema) {
    generated_rust.push_str("#[derive(Debug, Clone, Copy, PartialEq)]\n");
    generated_rust.push_str(&format!("pub struct {}Params {{\n", schema.id));

    for param in &schema.parameters {
        generated_rust.push_str(&format!("    pub {}: {},\n", param.id, param.type_name));
    }

    generated_rust.push_str("}\n\n");

    // Implement Default
    generated_rust.push_str(&format!("impl Default for {}Params {{\n", schema.id));
    generated_rust.push_str("    fn default() -> Self {\n");
    generated_rust.push_str("        Self {\n");
    for param in &schema.parameters {
        generated_rust.push_str(&format!(
            "            {}: {}f64,\n",
            param.id, param.default
        ));
    }
    generated_rust.push_str("        }\n");
    generated_rust.push_str("    }\n");
    generated_rust.push_str("}\n\n");

    // Implement TheoryDescribable
    generated_rust.push_str(&format!(
        "impl TheoryDescribable for {}Params {{\n",
        schema.id
    ));
    generated_rust.push_str("    fn theory_description(&self) -> String {\n");
    generated_rust.push_str(&format!("        \"{}\".to_string()\n", schema.description));
    generated_rust.push_str("    }\n");
    generated_rust.push_str("    fn theory_citation(&self) -> String {\n");
    generated_rust.push_str(&format!("        crate::citation_registry::CitationRegistry::register(\"{}\".to_string(), \"{}\".to_string());\n", schema.id, schema.citation));
    generated_rust.push_str(&format!("        \"{}\".to_string()\n", schema.citation));
    generated_rust.push_str("    }\n");
    generated_rust.push_str(
        "    fn available_descriptions(&self) -> std::collections::HashMap<String, String> {\n",
    );
    generated_rust.push_str("        let mut map = std::collections::HashMap::new();\n");
    generated_rust.push_str(&format!(
        "        map.insert(\"default\".to_string(), \"{}\".to_string());\n",
        schema.description
    ));
    generated_rust.push_str("        map\n");
    generated_rust.push_str("    }\n");
    generated_rust.push_str("}\n\n");
}

fn generate_latex_file(papers_dir: &Path, schema: &Schema) {
    let tex_path = papers_dir.join(format!("{}.tex", schema.id));
    let mut tex = String::new();
    tex.push_str("\\begin{table}[h]\n\\centering\n\\begin{tabular}{|l|c|r|}\n\\hline\n");
    tex.push_str("Parameter & Value Range & Unit \\\\\n\\hline\n");
    for param in &schema.parameters {
        let label = param.label.replace("+", "$+$").replace("_", "\\_");
        let unit = param.unit.replace("^2", "$^2$");
        tex.push_str(&format!(
            "{} & [{}, {}] & {} \\\\\n",
            label, param.min, param.max, unit
        ));
    }
    tex.push_str("\\hline\n\\end{tabular}\n");
    tex.push_str(&format!(
        "\\caption{{Parameters for the {}}}\n",
        schema.name
    ));
    tex.push_str("\\end{table}\n");

    let _ = fs::write(tex_path, tex);
}

fn generate_typed_model_command(generated_rust: &mut String, schemas: &[Schema]) {
    generated_rust.push_str("#[derive(Debug, Clone, Copy, PartialEq)]\n");
    generated_rust.push_str("pub enum TypedModelCommand {\n");
    for schema in schemas {
        generated_rust.push_str(&format!("    {}({}Params),\n", schema.id, schema.id));
    }
    generated_rust.push_str("}\n");
}
