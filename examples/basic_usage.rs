//! Basic usage example for pdf-converter
//! 
//! This example demonstrates how to convert images from a folder to PDF

use pdf_converter::{PdfConverter, PdfConfig};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();
    
    println!("🧪 PDF Converter - Basic Usage Example");
      // Create test images folder if it doesn't exist
    let test_folder = "examples/sample_images";
    if !Path::new(test_folder).exists() {
        println!("❌ Test folder doesn't exist: {}", test_folder);
        println!("Please create an 'examples/sample_images' folder and add some image files");
        return Ok(());
    }
    
    // Check test folder contents
    println!("📁 Checking test folder contents...");
    match std::fs::read_dir(test_folder) {
        Ok(entries) => {
            let mut count = 0;
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("  Found file: {}", entry.file_name().to_string_lossy());
                    count += 1;
                }
            }
            println!("  Total files found: {}", count);
            
            if count == 0 {
                println!("❌ No files found in test folder");
                return Ok(());
            }
        }
        Err(e) => {
            println!("❌ Cannot read folder: {}", e);
            return Ok(());
        }
    }
    
    // Example 1: Basic conversion with default settings
    println!("\n🔄 Example 1: Basic conversion with default settings");
    let converter = PdfConverter::new();
    let output_path = "basic_output.pdf";
    
    match converter.convert_folder_to_pdf(test_folder, output_path) {
        Ok(()) => {
            println!("✅ PDF generated successfully: {}", output_path);
            
            if Path::new(output_path).exists() {
                let metadata = std::fs::metadata(output_path)?;
                println!("📄 PDF file size: {} bytes", metadata.len());
            }
        },
        Err(e) => {
            println!("❌ PDF generation failed: {}", e);
        }
    }
    
    // Example 2: Custom configuration
    println!("\n🔄 Example 2: Custom configuration");
    let custom_config = PdfConfig {
        page_width_mm: 210.0,
        page_height_mm: 297.0,
        margin_mm: 10.0,  // Smaller margins
        dpi: 150.0,       // Lower DPI for smaller file size
        title: "Custom PDF Document".to_string(),
    };
    
    let custom_converter = PdfConverter::with_config(custom_config);
    let custom_output = "custom_output.pdf";
    
    match custom_converter.convert_folder_to_pdf(test_folder, custom_output) {
        Ok(()) => {
            println!("✅ Custom PDF generated successfully: {}", custom_output);
            
            if Path::new(custom_output).exists() {
                let metadata = std::fs::metadata(custom_output)?;
                println!("📄 Custom PDF file size: {} bytes", metadata.len());
            }
        },
        Err(e) => {
            println!("❌ Custom PDF generation failed: {}", e);
        }
    }
    
    // Example 3: Convert single image
    println!("\n🔄 Example 3: Convert single image");
    if let Ok(entries) = std::fs::read_dir(test_folder) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if let Some(ext_str) = extension.to_str() {
                            let ext_lower = ext_str.to_lowercase();                            if ["jpg", "jpeg", "png", "gif", "bmp", "webp"].contains(&ext_lower.as_str()) {
                                let single_output = "single_image.pdf";
                                match converter.convert_image_to_pdf(path.to_str().unwrap(), single_output) {
                                    Ok(()) => {
                                        println!("✅ Single image PDF generated: {} -> {}", 
                                            path.display(), single_output);
                                        break;
                                    },
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
    
    println!("\n🏁 Examples completed!");
    Ok(())
}
