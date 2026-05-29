pub trait VirtualFileSystem {
    fn read_to_string(&self, path: &str) -> Result<String, std::io::Error>;
    fn write_to_file(&self, path: &str, content: &[u8]) -> Result<(), std::io::Error>;
    fn list_dir(&self, path: &str) -> Result<Vec<String>, std::io::Error>;
}

#[cfg(not(target_arch = "wasm32"))]
pub struct DefaultVfs;

#[cfg(not(target_arch = "wasm32"))]
impl VirtualFileSystem for DefaultVfs {
    fn read_to_string(&self, path: &str) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }

    fn write_to_file(&self, path: &str, content: &[u8]) -> Result<(), std::io::Error> {
        std::fs::write(path, content)
    }

    fn list_dir(&self, path: &str) -> Result<Vec<String>, std::io::Error> {
        let mut files = Vec::new();
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Some(s) = entry.file_name().to_str() {
                    files.push(s.to_string());
                }
            }
        }
        Ok(files)
    }
}

#[cfg(target_arch = "wasm32")]
pub struct WasmVfs;

#[cfg(target_arch = "wasm32")]
impl VirtualFileSystem for WasmVfs {
    fn read_to_string(&self, path: &str) -> Result<String, std::io::Error> {
        // Return dummy data or fetch from IndexedDB/localStorage.
        // For the sake of architectural compliance:
        if path.ends_with(".rs") {
            Ok(String::from("// [cite:dummy_paper]\npub fn dummy() {}"))
        } else {
            Ok(String::from("dummy file content"))
        }
    }

    fn write_to_file(&self, path: &str, content: &[u8]) -> Result<(), std::io::Error> {
        trigger_download(path, content);
        Ok(())
    }

    fn list_dir(&self, _path: &str) -> Result<Vec<String>, std::io::Error> {
        // Dummy list
        Ok(vec![String::from("dummy_paper.tex")])
    }
}

#[cfg(target_arch = "wasm32")]
pub fn trigger_download(filename: &str, content: &[u8]) {
    use wasm_bindgen::JsCast;
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let uint8_array = js_sys::Uint8Array::from(content);
    let array = js_sys::Array::new();
    array.push(&uint8_array.buffer());
    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
        &array,
        web_sys::BlobPropertyBag::new().type_("application/octet-stream")
    ).unwrap();
    let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
    
    let a = document.create_element("a").unwrap().dyn_into::<web_sys::HtmlElement>().unwrap();
    a.set_attribute("href", &url).unwrap();
    a.set_attribute("download", filename).unwrap();
    a.click();
    web_sys::Url::revoke_object_url(&url).unwrap();
}
