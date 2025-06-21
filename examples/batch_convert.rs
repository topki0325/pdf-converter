//! Batch conversion example for pdf-converter
//! 
//! This example demonstrates batch processing of multiple folders

use pdf_converter::{PdfConverter, PdfConfig};
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    println!("🧪 PDF Converter - Batch Processing Example");
    
    // Create converter with optimized settings for batch processing
    let batch_config = PdfConfig {
        page_width_mm: 210.0,
        page_height_mm: 297.0,
        margin_mm: 15.0,
        dpi: 200.0,  // Medium quality for faster processing
        title: "Batch Converted PDF".to_string(),
    };
    
    let converter = PdfConverter::with_config(batch_config);
      // Define folders to process
    let folders_to_process = [
        "examples/sample_images",
    ];
    
    let mut processed_count = 0;
    let mut success_count = 0;
    
    println!("📁 Processing {} folders...\n", folders_to_process.len());
    
    for (index, folder) in folders_to_process.iter().enumerate() {
        processed_count += 1;
        
        println!("🔄 Processing folder {}/{}: {}", index + 1, folders_to_process.len(), folder);
        
        let folder_path = Path::new(folder);
        if !folder_path.exists() {
            println!("  ⏭️  Skipping non-existent folder: {}", folder);
            continue;
        }
        
        // Count images in folder
        match count_images_in_folder(folder_path) {
            Ok(count) => {
                if count == 0 {
                    println!("  ⏭️  Skipping empty folder: {}", folder);
                    continue;
                }
                println!("  📸 Found {} images", count);
            }
            Err(e) => {
                println!("  ❌ Error reading folder: {}", e);
                continue;
            }
        }
        
        // Generate output filename based on folder name
        let output_filename = generate_output_filename(folder);
        
        // Convert folder to PDF
        match converter.convert_folder_to_pdf(folder, &output_filename) {
            Ok(()) => {
                success_count += 1;
                println!("  ✅ Success: {}", output_filename);
                
                // Show file size
                if let Ok(metadata) = std::fs::metadata(&output_filename) {
                    println!("     📄 Size: {} KB", metadata.len() / 1024);
                }
            }
            Err(e) => {
                println!("  ❌ Failed: {}", e);
            }
        }
        
        println!(); // Empty line for readability
    }
    
    // Summary
    println!("📊 Batch Processing Summary:");
    println!("   Total folders processed: {}", processed_count);
    println!("   Successful conversions: {}", success_count);
    println!("   Failed conversions: {}", processed_count - success_count);
    
    if success_count > 0 {
        println!("\n🎉 Batch processing completed successfully!");
        
        // List all generated PDFs
        println!("\n📄 Generated PDF files:");
        for folder in &folders_to_process {
            let output_filename = generate_output_filename(folder);
            if Path::new(&output_filename).exists() {
                if let Ok(metadata) = std::fs::metadata(&output_filename) {
                    println!("   {} ({} KB)", output_filename, metadata.len() / 1024);
                }
            }
        }
    } else {
        println!("\n😞 No PDFs were generated successfully.");
    }
    
    Ok(())
}

/// Count the number of image files in a folder
fn count_images_in_folder(folder: &Path) -> Result<usize, std::io::Error> {
    let image_extensions = ["jpg", "jpeg", "png", "gif", "bmp", "webp"];
    let mut count = 0;
    
    for entry in std::fs::read_dir(folder)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            if let Some(extension) = entry.path().extension() {
                if let Some(ext_str) = extension.to_str() {
                    if image_extensions.contains(&ext_str.to_lowercase().as_str()) {
                        count += 1;
                    }
                }
            }
        }
    }
    
    Ok(count)
}

/// Generate output filename based on input folder path
fn generate_output_filename(folder: &str) -> String {
    let safe_name = folder
        .replace("/", "_")
        .replace("\\", "_")
        .replace(" ", "_")
        .replace(":", "");
    
    format!("batch_{}.pdf", safe_name)
}
