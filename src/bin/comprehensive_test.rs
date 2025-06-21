//! 全面的PDF转换测试程序
//! 测试pdf-converter的各种功能

use pdf_converter::{PdfConverter, PdfConfig};
use std::path::Path;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 PDF转换器全面测试");
    println!("{}", "=".repeat(50));
    
    // 测试1: 基本单图片转换
    test_single_image_conversion()?;
    
    // 测试2: 自定义配置转换
    test_custom_config_conversion()?;
    
    // 测试3: 批量转换（选择部分图片）
    test_batch_conversion()?;
    
    // 测试4: 性能测试
    test_performance()?;
    
    // 显示所有生成的PDF文件
    show_generated_pdfs();
    
    println!("\n🎉 所有测试完成!");
    Ok(())
}

fn test_single_image_conversion() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔄 测试1: 单图片转换");
    
    let converter = PdfConverter::new();
    let test_images = [
        ("test_images/1.jpg", "comprehensive_test_1.pdf"),
        ("test_images/Page-01.jpg", "comprehensive_test_page01.pdf"),
        ("test_images/Page-02.jpg", "comprehensive_test_page02.pdf"),
    ];
    
    for (input, output) in &test_images {
        if Path::new(input).exists() {
            let start_time = Instant::now();
            match converter.convert_image_to_pdf(input, output) {
                Ok(()) => {
                    let duration = start_time.elapsed();
                    println!("  ✅ {} -> {} (用时: {:?})", input, output, duration);
                    
                    if let Ok(metadata) = std::fs::metadata(output) {
                        println!("     📄 文件大小: {} KB", metadata.len() / 1024);
                    }
                }
                Err(e) => {
                    println!("  ❌ {} 转换失败: {}", input, e);
                }
            }
        } else {
            println!("  ⏭️  跳过不存在的文件: {}", input);
        }
    }
    
    Ok(())
}

fn test_custom_config_conversion() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔄 测试2: 自定义配置转换");
    
    // 高质量配置
    let high_quality_config = PdfConfig {
        page_width_mm: 210.0,
        page_height_mm: 297.0,
        margin_mm: 15.0,
        dpi: 300.0,
        title: "高质量PDF文档".to_string(),
    };
    
    // 快速配置（低DPI）
    let fast_config = PdfConfig {
        page_width_mm: 210.0,
        page_height_mm: 297.0,
        margin_mm: 20.0,
        dpi: 150.0,
        title: "快速生成PDF文档".to_string(),
    };
    
    let configs = [
        (high_quality_config, "comprehensive_high_quality.pdf", "高质量"),
        (fast_config, "comprehensive_fast.pdf", "快速"),
    ];
    
    for (config, output, description) in configs {
        let converter = PdfConverter::with_config(config);
        
        if Path::new("test_images/1.jpg").exists() {
            let start_time = Instant::now();
            match converter.convert_image_to_pdf("test_images/1.jpg", &output) {
                Ok(()) => {
                    let duration = start_time.elapsed();
                    println!("  ✅ {} 配置: {} (用时: {:?})", description, output, duration);
                    
                    if let Ok(metadata) = std::fs::metadata(&output) {
                        println!("     📄 文件大小: {} KB", metadata.len() / 1024);
                    }
                }
                Err(e) => {
                    println!("  ❌ {} 配置转换失败: {}", description, e);
                }
            }
        }
    }
    
    Ok(())
}

fn test_batch_conversion() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔄 测试3: 批量转换（选择简单文件名）");
    
    let converter = PdfConverter::new();
    
    // 选择一些简单文件名的图片进行批量转换
    let simple_images = [
        "test_images/1.jpg",
        "test_images/Page-01.jpg", 
        "test_images/Page-02.jpg",
        "test_images/JTE39.jpg",
    ];
    
    let mut existing_images = Vec::new();
    for image_path in &simple_images {
        if Path::new(image_path).exists() {
            existing_images.push(image_path);
        }
    }
    
    if existing_images.is_empty() {
        println!("  ⚠️  没有找到合适的图片进行批量测试");
        return Ok(());
    }
    
    println!("  📸 找到 {} 张图片用于批量转换", existing_images.len());
    for image in &existing_images {
        println!("     - {}", image);
    }
    
    let output = "comprehensive_batch.pdf";
    let start_time = Instant::now();
    
    // 由于API限制，我们创建一个临时文件夹来测试批量转换
    let temp_dir = "temp_batch_test";
    std::fs::create_dir_all(temp_dir)?;
    
    // 复制选中的图片到临时文件夹
    for (i, &image_path) in existing_images.iter().enumerate() {
        let filename = format!("batch_image_{:02}.jpg", i + 1);
        let dest_path = format!("{}/{}", temp_dir, filename);
        std::fs::copy(image_path, &dest_path)?;
    }
    
    match converter.convert_folder_to_pdf(temp_dir, output) {
        Ok(()) => {
            let duration = start_time.elapsed();
            println!("  ✅ 批量转换成功: {} (用时: {:?})", output, duration);
            
            if let Ok(metadata) = std::fs::metadata(output) {
                println!("     📄 文件大小: {} KB", metadata.len() / 1024);
                println!("     📑 包含 {} 张图片", existing_images.len());
            }
        }
        Err(e) => {
            println!("  ❌ 批量转换失败: {}", e);
        }
    }
    
    // 清理临时文件夹
    std::fs::remove_dir_all(temp_dir).ok();
    
    Ok(())
}

fn test_performance() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔄 测试4: 性能测试");
    
    if !Path::new("test_images/1.jpg").exists() {
        println!("  ⚠️  跳过性能测试 - 测试图片不存在");
        return Ok(());
    }
    
    let dpi_configs = [
        (100.0, "低质量"),
        (150.0, "中等质量"),
        (200.0, "高质量"),
        (300.0, "超高质量"),
    ];
    
    for (dpi, description) in &dpi_configs {
        let config = PdfConfig {
            page_width_mm: 210.0,
            page_height_mm: 297.0,
            margin_mm: 20.0,
            dpi: *dpi,
            title: format!("{}测试", description),
        };
          let converter = PdfConverter::with_config(config);
        let output = format!("comprehensive_perf_{}.pdf", *dpi as i32);
        
        let start_time = Instant::now();
        match converter.convert_image_to_pdf("test_images/1.jpg", &output) {
            Ok(()) => {
                let duration = start_time.elapsed();
                println!("  ✅ {} ({} DPI): {} (用时: {:?})", description, dpi, output, duration);
                
                if let Ok(metadata) = std::fs::metadata(&output) {
                    println!("     📄 文件大小: {} KB", metadata.len() / 1024);
                }
            }
            Err(e) => {
                println!("  ❌ {} ({} DPI) 转换失败: {}", description, dpi, e);
            }
        }
    }
    
    Ok(())
}

fn show_generated_pdfs() {
    println!("\n📋 生成的PDF文件:");
    
    let current_dir = std::env::current_dir().unwrap_or_default();
    let mut total_files = 0;
    let mut total_size = 0;
    
    if let Ok(entries) = std::fs::read_dir(&current_dir) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.ends_with(".pdf") && name.starts_with("comprehensive") {
                            let size_kb = metadata.len() / 1024;
                            let name_str = name.replace("comprehensive_", "").replace(".pdf", "");
                            println!("  📄 {} ({} KB)", name_str, size_kb);
                            total_files += 1;
                            total_size += size_kb;
                        }
                    }
                }
            }
        }
    }
    
    if total_files > 0 {
        println!("  📊 总计: {} 个文件, {} KB", total_files, total_size);
    } else {
        println!("  ℹ️  没有找到生成的PDF文件");
    }
}
