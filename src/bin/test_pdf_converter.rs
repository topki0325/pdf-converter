//! Standalone test program for pdf-converter
//! 
//! This program tests the PDF converter functionality with real images
//! Run with: cargo run --bin test_pdf_converter

use pdf_converter::{PdfConverter, PdfConfig};
use std::path::Path;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Note: env_logger is only available in tests, not in binary programs
    // env_logger::init();
    
    println!("🧪 PDF Converter Test Program");
    println!("{}", "=".repeat(50));
    
    // Check if test images exist
    let test_folder = "../test_images";
    if !Path::new(test_folder).exists() {
        println!("❌ Test images folder not found: {}", test_folder);
        println!("Please ensure the test_images folder exists in the parent directory");
        return Ok(());
    }
    
    // List available test images
    println!("📁 Available test images:");
    let mut image_count = 0;
    match std::fs::read_dir(test_folder) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(extension) = path.extension() {
                            if let Some(ext_str) = extension.to_str() {
                                let ext_lower = ext_str.to_lowercase();
                                if ["jpg", "jpeg", "png", "gif", "bmp", "webp"].contains(&ext_lower.as_str()) {
                                    println!("  📷 {} ({})", 
                                        entry.file_name().to_string_lossy(), 
                                        ext_str.to_uppercase());
                                    image_count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Cannot read test folder: {}", e);
            return Ok(());
        }
    }
    
    if image_count == 0 {
        println!("❌ No image files found in test folder");
        return Ok(());
    }
    
    println!("   Total: {} image files found\n", image_count);
    
    // Test 1: Convert folder to PDF with default settings
    println!("🔄 Test 1: Convert folder to PDF (default settings)");
    test_folder_conversion_default(test_folder)?;
    
    // Test 2: Convert folder to PDF with custom settings
    println!("\n🔄 Test 2: Convert folder to PDF (custom settings)");
    test_folder_conversion_custom(test_folder)?;
    
    // Test 3: Convert single image
    println!("\n🔄 Test 3: Convert single image");
    test_single_image_conversion(test_folder)?;
    
    // Test 4: Performance test
    println!("\n🔄 Test 4: Performance test");
    test_performance(test_folder)?;
    
    // Test 5: Different image formats
    println!("\n🔄 Test 5: Test different image formats");
    test_different_formats(test_folder)?;
    
    println!("\n🎉 All tests completed successfully!");
    println!("\n📄 Generated PDF files:");
    list_generated_pdfs();
    
    Ok(())
}

fn test_folder_conversion_default(test_folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    let converter = PdfConverter::new();
    let output_path = "test_folder_default.pdf";
    
    let start_time = Instant::now();
    match converter.convert_folder_to_pdf(test_folder, output_path) {
        Ok(()) => {
            let duration = start_time.elapsed();
            println!("  ✅ Success: {} (took {:?})", output_path, duration);
            
            if Path::new(output_path).exists() {
                let metadata = std::fs::metadata(output_path)?;
                println!("     📄 File size: {} KB", metadata.len() / 1024);
            }
        }
        Err(e) => {
            println!("  ❌ Failed: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}

fn test_folder_conversion_custom(test_folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    let custom_config = PdfConfig {
        page_width_mm: 210.0,
        page_height_mm: 297.0,
        margin_mm: 10.0,  // Smaller margins
        dpi: 150.0,       // Lower DPI for faster processing
        title: "Custom Settings Test PDF".to_string(),
    };
    
    let converter = PdfConverter::with_config(custom_config);
    let output_path = "test_folder_custom.pdf";
    
    let start_time = Instant::now();
    match converter.convert_folder_to_pdf(test_folder, output_path) {
        Ok(()) => {
            let duration = start_time.elapsed();
            println!("  ✅ Success: {} (took {:?})", output_path, duration);
            
            if Path::new(output_path).exists() {
                let metadata = std::fs::metadata(output_path)?;
                println!("     📄 File size: {} KB (should be smaller due to lower DPI)", metadata.len() / 1024);
            }
        }
        Err(e) => {
            println!("  ❌ Failed: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}

fn test_single_image_conversion(test_folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    let converter = PdfConverter::new();
    
    // Find the first image file
    match std::fs::read_dir(test_folder) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(extension) = path.extension() {
                            if let Some(ext_str) = extension.to_str() {
                                let ext_lower = ext_str.to_lowercase();
                                if ["jpg", "jpeg", "png", "gif", "bmp", "webp"].contains(&ext_lower.as_str()) {
                                    let output_path = "test_single_image.pdf";
                                    
                                    let start_time = Instant::now();
                                    match converter.convert_image_to_pdf(path.clone(), output_path.into()) {
                                        Ok(()) => {
                                            let duration = start_time.elapsed();
                                            println!("  ✅ Success: {} -> {} (took {:?})", 
                                                path.display(), output_path, duration);
                                            
                                            if Path::new(output_path).exists() {
                                                let metadata = std::fs::metadata(output_path)?;
                                                println!("     📄 File size: {} KB", metadata.len() / 1024);
                                            }
                                            return Ok(());
                                        }
                                        Err(e) => {
                                            println!("  ❌ Failed: {}", e);
                                            return Err(e.into());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("  ❌ Cannot read test folder: {}", e);
            return Err(e.into());
        }
    }
    
    println!("  ⚠️  No suitable image found for single image test");
    Ok(())
}

fn test_performance(test_folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Test with high-quality settings
    let high_quality_config = PdfConfig {
        page_width_mm: 210.0,
        page_height_mm: 297.0,
        margin_mm: 15.0,
        dpi: 300.0,  // High DPI
        title: "High Quality Performance Test".to_string(),
    };
    
    // Test with fast settings
    let fast_config = PdfConfig {
        page_width_mm: 210.0,
        page_height_mm: 297.0,
        margin_mm: 15.0,
        dpi: 100.0,  // Low DPI for speed
        title: "Fast Performance Test".to_string(),
    };
    
    // High quality test
    let high_quality_converter = PdfConverter::with_config(high_quality_config);
    let high_quality_output = "test_performance_high.pdf";
    
    let start_time = Instant::now();
    match high_quality_converter.convert_folder_to_pdf(test_folder, high_quality_output) {
        Ok(()) => {
            let duration = start_time.elapsed();
            println!("  ✅ High quality (300 DPI): {:?}", duration);
            
            if Path::new(high_quality_output).exists() {
                let metadata = std::fs::metadata(high_quality_output)?;
                println!("     📄 File size: {} KB", metadata.len() / 1024);
            }
        }
        Err(e) => {
            println!("  ❌ High quality test failed: {}", e);
        }
    }
    
    // Fast test
    let fast_converter = PdfConverter::with_config(fast_config);
    let fast_output = "test_performance_fast.pdf";
    
    let start_time = Instant::now();
    match fast_converter.convert_folder_to_pdf(test_folder, fast_output) {
        Ok(()) => {
            let duration = start_time.elapsed();
            println!("  ✅ Fast quality (100 DPI): {:?}", duration);
            
            if Path::new(fast_output).exists() {
                let metadata = std::fs::metadata(fast_output)?;
                println!("     📄 File size: {} KB (should be smaller)", metadata.len() / 1024);
            }
        }
        Err(e) => {
            println!("  ❌ Fast test failed: {}", e);
        }
    }
    
    Ok(())
}

fn test_different_formats(test_folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    let converter = PdfConverter::new();
    let format_extensions = ["jpg", "jpeg", "png", "gif", "bmp", "webp"];
    
    match std::fs::read_dir(test_folder) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(extension) = path.extension() {
                            if let Some(ext_str) = extension.to_str() {
                                let ext_lower = ext_str.to_lowercase();
                                if format_extensions.contains(&ext_lower.as_str()) {
                                    let output_path = format!("test_format_{}.pdf", ext_lower);
                                    
                                    let start_time = Instant::now();
                                    match converter.convert_image_to_pdf(path.clone(), output_path.as_str().into()) {
                                        Ok(()) => {
                                            let duration = start_time.elapsed();
                                            println!("  ✅ {}: {} -> {} (took {:?})", 
                                                ext_str.to_uppercase(),
                                                path.file_name().unwrap().to_string_lossy(),
                                                output_path, 
                                                duration);
                                            
                                            if Path::new(&output_path).exists() {
                                                let metadata = std::fs::metadata(&output_path)?;
                                                println!("       📄 Size: {} KB", metadata.len() / 1024);
                                            }
                                        }
                                        Err(e) => {
                                            println!("  ❌ {} failed: {}", ext_str.to_uppercase(), e);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("  ❌ Cannot read test folder: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}

fn list_generated_pdfs() {
    let pdf_files = [
        "test_folder_default.pdf",
        "test_folder_custom.pdf",
        "test_single_image.pdf",
        "test_performance_high.pdf",
        "test_performance_fast.pdf",
        "test_format_jpg.pdf",
        "test_format_jpeg.pdf",
        "test_format_png.pdf",
        "test_format_gif.pdf",
        "test_format_bmp.pdf",
        "test_format_webp.pdf",
    ];
    
    for pdf_file in &pdf_files {
        if Path::new(pdf_file).exists() {
            if let Ok(metadata) = std::fs::metadata(pdf_file) {
                println!("  📄 {} ({} KB)", pdf_file, metadata.len() / 1024);
            }
        }
    }
}
