use lopdf::Document;
use pdf::file::File as PdfFile;
use pdf::object::Object;
use pdf::object::StringFormat;
use pdf::object::StringObject;
use pdf::primitive::Primitive;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::fs;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn encrypt_pdfs(input_files: Vec<String>, password: String) -> Result<(), String> {
    let output_dir = Path::new("output");
    if !output_dir.exists() {
        fs::create_dir(output_dir).map_err(|e| e.to_string())?;
    }

    for input_file in input_files {
        let mut doc = Document::load(&input_file).map_err(|e| e.to_string())?;
        doc.encrypt(password.as_bytes(), password.as_bytes(), lopdf::crypt::Security::default())
            .map_err(|e| e.to_string())?;
        let output_file = output_dir.join(Path::new(&input_file).file_name().unwrap());
        doc.save(output_file).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn decrypt_pdfs(input_files: Vec<String>, password: String) -> Result<(), String> {
    let output_dir = Path::new("output");
    if !output_dir.exists() {
        fs::create_dir(output_dir).map_err(|e| e.to_string())?;
    }

    for input_file in input_files {
        let mut doc = Document::load(&input_file).map_err(|e| e.to_string())?;
        doc.decrypt(password.as_bytes()).map_err(|e| e.to_string())?;
        let output_file = output_dir.join(Path::new(&input_file).file_name().unwrap());
        doc.save(output_file).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn merge_pdfs(input_files: Vec<String>, output_file: String) -> Result<(), String> {
    let output_dir = Path::new("output");
    if !output_dir.exists() {
        fs::create_dir(output_dir).map_err(|e| e.to_string())?;
    }

    let mut output_doc = Document::with_version("1.5");
    for input_file in input_files {
        let input_doc = Document::load(&input_file).map_err(|e| e.to_string())?;
        for page_id in input_doc.page_iter() {
            let page = input_doc.get_page(page_id).map_err(|e| e.to_string())?;
            output_doc.add_page(page, input_doc.get_page_content(page_id).map_err(|e| e.to_string())?);
        }
    }
    let output_path = output_dir.join(output_file);
    output_doc.save(output_path).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, encrypt_pdfs, decrypt_pdfs, merge_pdfs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
