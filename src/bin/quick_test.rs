//! Quick test for pdf-converter functionality
//! This is a minimal test to verify basic functionality

use pdf_converter::PdfConverter;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Quick PDF Converter Test");
    println!("{}", "=".repeat(30));
      // Check if test images exist
    let test_folder = "test_images";
    if !Path::new(test_folder).exists() {
        println!("❌ Test folder not found: {}", test_folder);
        println!("Creating a simple test...");
        
        // If no test folder, try to find any image in current directory
        match find_any_image(".") {
            Some(image_path) => {
                println!("📷 Found image: {}", image_path);
                test_single_image(&image_path)?;
            }
            None => {
                println!("❌ No images found to test with");
                println!("Please place some image files in the test_images folder or current directory");
                return Ok(());
            }
        }
    } else {
        // Test with folder
        test_folder_conversion(test_folder)?;
    }
    
    println!("✅ Quick test completed!");
    Ok(())
}

fn test_folder_conversion(folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Testing folder conversion: {}", folder);
    
    let converter = PdfConverter::new();
    let output = "quick_test_folder.pdf";
    
    match converter.convert_folder_to_pdf(folder, output) {
        Ok(()) => {
            println!("✅ Folder conversion successful: {}", output);
            
            if Path::new(output).exists() {
                let metadata = std::fs::metadata(output)?;
                println!("📄 PDF size: {} KB", metadata.len() / 1024);
            }
        }
        Err(e) => {
            println!("❌ Folder conversion failed: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}


fn test_single_image(image_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Testing single image conversion: {}", image_path);
    
    let converter = PdfConverter::new();
    let output = "quick_test_single.pdf";
    
    match converter.convert_image_to_pdf(image_path, output) {
        Ok(()) => {
            println!("✅ Single image conversion successful: {}", output);
            
            if Path::new(output).exists() {
                let metadata = std::fs::metadata(output)?;
                println!("📄 PDF size: {} KB", metadata.len() / 1024);
            }
        }
        Err(e) => {
            println!("❌ Single image conversion failed: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}

fn find_any_image(dir: &str) -> Option<String> {
    let image_extensions = ["jpg", "jpeg", "png", "gif", "bmp", "webp"];
    
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if let Some(ext_str) = extension.to_str() {
                            if image_extensions.contains(&ext_str.to_lowercase().as_str()) {
                                return Some(path.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    
    None
}
