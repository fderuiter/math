pub trait VirtualFileSystem {
    fn read_to_string(&self, path: &str) -> Result<String, std::io::Error>;
    fn write_to_file(&self, path: &str, content: &[u8]) -> Result<(), std::io::Error>;
    fn list_dir(&self, path: &str) -> Result<Vec<String>, std::io::Error>;
}

pub mod vfs_data {
    include!(concat!(env!("OUT_DIR"), "/vfs_data.rs"));
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
        if let Some(content) = vfs_data::get_file_content(path) {
            Ok(content.to_string())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found in VFS",
            ))
        }
    }

    fn write_to_file(&self, path: &str, content: &[u8]) -> Result<(), std::io::Error> {
        trigger_download(path, content);
        Ok(())
    }

    fn list_dir(&self, path: &str) -> Result<Vec<String>, std::io::Error> {
        if let Some(children) = vfs_data::get_dir_children(path) {
            Ok(children.iter().map(|s| s.to_string()).collect())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Directory not found in VFS",
            ))
        }
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
    let options = web_sys::BlobPropertyBag::new();
    options.set_type("application/octet-stream");
    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&array, &options).unwrap();
    let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

    let a = document
        .create_element("a")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    a.set_attribute("href", &url).unwrap();
    a.set_attribute("download", filename).unwrap();
    a.click();
    web_sys::Url::revoke_object_url(&url).unwrap();
}
