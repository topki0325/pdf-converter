//! Simple test for pdf-converter
//! This is a minimal test to verify basic functionality

use pdf_converter::PdfConverter;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Simple PDF Converter Test");
    println!("{}", "=".repeat(30));
    
    // Check if test images exist
    let test_folder = "../test_images";
    if !Path::new(test_folder).exists() {
        println!("❌ Test folder not found: {}", test_folder);
        println!("Please ensure the test_images folder exists");
        return Ok(());
    }
    
    // Test 1: Convert folder to PDF
    println!("🔄 Testing folder conversion...");
    let converter = PdfConverter::new();
    let output_path = "simple_test_output.pdf";
    
    match converter.convert_folder_to_pdf(test_folder, output_path) {
        Ok(()) => {
            println!("✅ Folder conversion successful: {}", output_path);
            
            if Path::new(output_path).exists() {
                let metadata = std::fs::metadata(output_path)?;
                println!("📄 PDF size: {} KB", metadata.len() / 1024);
            }
        }
        Err(e) => {
            println!("❌ Folder conversion failed: {}", e);
        }
    }
    
    // Test 2: Find and convert a single image
    println!("\n🔄 Testing single image conversion...");
    if let Ok(entries) = std::fs::read_dir(test_folder) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if let Some(ext_str) = extension.to_str() {
                            let ext_lower = ext_str.to_lowercase();
                            if ["jpg", "jpeg", "png"].contains(&ext_lower.as_str()) {
                                let single_output = "simple_single_test.pdf";
                                match converter.convert_image_to_pdf(path.as_path(), std::path::Path::new(single_output)) {
                                    Ok(()) => {
                                        println!("✅ Single image conversion successful: {} -> {}", 
                                            path.display(), single_output);
                                        if Path::new(single_output).exists() {
                                            let metadata = std::fs::metadata(single_output)?;
                                            println!("📄 PDF size: {} KB", metadata.len() / 1024);
                                        }
                                        break;
                                    }
                                    Err(e) => {
                                        println!("❌ Single image conversion failed: {}", e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("\n🏁 Simple test completed!");
    Ok(())
}
