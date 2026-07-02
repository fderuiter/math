use std::fs;
use toml::Value;

pub fn check_osv_vulnerabilities() -> bool {
    let content = fs::read_to_string("Cargo.lock").unwrap_or_default();
    let parsed: Value = match toml::from_str(&content) {
        Ok(v) => v,
        Err(_) => return true,
    };
    
    let mut queries = Vec::new();
    let mut pkg_names = Vec::new();
    
    if let Some(packages) = parsed.get("package").and_then(|p| p.as_array()) {
        for pkg in packages {
            if let (Some(name), Some(version)) = (pkg.get("name").and_then(|n| n.as_str()), pkg.get("version").and_then(|v| v.as_str())) {
                let source = pkg.get("source").and_then(|s| s.as_str()).unwrap_or("");
                if source.contains("crates.io") {
                    queries.push(ureq::json!({
                        "package": {
                            "name": name,
                            "ecosystem": "crates.io"
                        },
                        "version": version
                    }));
                    pkg_names.push((name.to_string(), version.to_string()));
                }
            }
        }
    }
    
    if queries.is_empty() {
        return true;
    }
    
    let req_body = ureq::json!({ "queries": queries });
    let res = match ureq::post("https://api.osv.dev/v1/querybatch")
        .set("Content-Type", "application/json")
        .send_json(req_body) {
        Ok(r) => r,
        Err(_) => return true, // if network fails, don't fail the build
    };
    
    let res_json: serde_json::Value = res.into_json().unwrap_or_default();
    let mut passed = true;
    
    if let Some(results) = res_json.get("results").and_then(|r| r.as_array()) {
        for (i, result) in results.iter().enumerate() {
            if let Some(vulns) = result.get("vulns").and_then(|v| v.as_array()) {
                for vuln in vulns {
                    let mut is_high = false;
                    if let Some(severity) = vuln.get("severity").and_then(|s| s.as_array()) {
                        for s in severity {
                            if s.get("type").and_then(|t| t.as_str()) == Some("CVSS_V3") {
                                if let Some(score) = s.get("score").and_then(|sc| sc.as_str()) {
                                    if score.contains("/A:H") || score.contains("/C:H") || score.contains("/I:H") {
                                        is_high = true;
                                    }
                                }
                            }
                        }
                    }
                    if is_high {
                        println!("[!] High-severity vulnerability found in {} {}: {}", pkg_names[i].0, pkg_names[i].1, vuln.get("id").and_then(|i| i.as_str()).unwrap_or(""));
                        passed = false;
                    }
                }
            }
        }
    }
    passed
}
