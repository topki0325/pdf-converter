//! PDF converter implementation

use std::path::{Path, PathBuf};
use std::io::BufWriter;
use printpdf::*;
use ::image::GenericImageView;
use crate::error::{PdfError, Result};
use crate::{A4_WIDTH_MM, A4_HEIGHT_MM, DEFAULT_MARGIN_MM, DEFAULT_DPI};

/// Configuration for PDF conversion
#[derive(Debug, Clone)]
pub struct PdfConfig {
    /// Page width in millimeters
    pub page_width_mm: f32,
    /// Page height in millimeters
    pub page_height_mm: f32,
    /// Page margins in millimeters
    pub margin_mm: f32,
    /// DPI for image conversion
    pub dpi: f32,
    /// PDF document title
    pub title: String,
}

impl Default for PdfConfig {
    fn default() -> Self {
        Self {
            page_width_mm: A4_WIDTH_MM,
            page_height_mm: A4_HEIGHT_MM,
            margin_mm: DEFAULT_MARGIN_MM,
            dpi: DEFAULT_DPI,
            title: "Generated PDF".to_string(),
        }
    }
}

/// PDF converter for images
pub struct PdfConverter {
    config: PdfConfig,
}

impl Default for PdfConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfConverter {
    /// Create a new PDF converter with default settings
    pub fn new() -> Self {
        Self {
            config: PdfConfig::default(),
        }
    }

    /// Create a new PDF converter with custom configuration
    pub fn with_config(config: PdfConfig) -> Self {
        Self { config }
    }

    /// Convert all images in a folder to a single PDF
    /// 
    /// # Arguments
    /// 
    /// * `folder_path` - Path to folder containing images
    /// * `output_path` - Path where the PDF will be saved
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use pdf_converter::PdfConverter;
    /// 
    /// let converter = PdfConverter::new();
    /// converter.convert_folder_to_pdf("images/", "output.pdf")?;
    /// ```
    pub fn convert_folder_to_pdf<P: AsRef<Path>>(&self, folder_path: P, output_path: P) -> Result<()> {
        let folder = folder_path.as_ref();
        let output = output_path.as_ref();
        
        log::info!("📄 开始生成PDF: {} -> {}", folder.display(), output.display());
        
        if !folder.exists() || !folder.is_dir() {
            return Err(PdfError::InvalidPath(folder.display().to_string()));
        }

        // 收集所有图片文件
        let mut image_files = self.collect_image_files(folder)?;
        
        if image_files.is_empty() {
            return Err(PdfError::NoImagesFound(folder.display().to_string()));
        }

        // 按文件名排序
        image_files.sort();
        log::info!("📸 找到 {} 张图片，开始生成PDF", image_files.len());

        self.convert_images_to_pdf(&image_files, output)
    }

    /// Convert a single image to PDF
    /// 
    /// # Arguments
    /// 
    /// * `image_path` - Path to the image file
    /// * `output_path` - Path where the PDF will be saved
    pub fn convert_image_to_pdf<P: AsRef<Path>>(&self, image_path: P, output_path: P) -> Result<()> {
        let image = image_path.as_ref();
        let output = output_path.as_ref();
        
        if !image.exists() {
            return Err(PdfError::InvalidPath(image.display().to_string()));
        }

        log::info!("📄 转换单张图片到PDF: {} -> {}", image.display(), output.display());
        
        let images = vec![image.to_path_buf()];
        self.convert_images_to_pdf(&images, output)
    }

    /// Convert multiple specific images to PDF
    /// 
    /// # Arguments
    /// 
    /// * `image_paths` - Vector of paths to image files
    /// * `output_path` - Path where the PDF will be saved
    pub fn convert_images_to_pdf<P: AsRef<Path>>(&self, image_paths: &[PathBuf], output_path: P) -> Result<()> {
        let output = output_path.as_ref();
        
        if image_paths.is_empty() {
            return Err(PdfError::Custom("No images provided".to_string()));
        }        // 创建PDF文档
        let (doc, page1, layer1) = printpdf::PdfDocument::new(
            &self.config.title,
            printpdf::Mm(self.config.page_width_mm),
            printpdf::Mm(self.config.page_height_mm),
            "Layer 1"
        );
        
        // 处理第一张图片
        let current_layer = doc.get_page(page1).get_layer(layer1);
        self.add_image_to_pdf_layer(&current_layer, &image_paths[0])?;

        // 处理剩余图片（每张图片一页）
        for (index, image_path) in image_paths.iter().enumerate().skip(1) {
            log::info!("  处理第 {}/{} 张图片: {}", index + 1, image_paths.len(), 
                image_path.file_name().unwrap_or_default().to_string_lossy());            // 添加新页面
            let (page_index, layer_index) = doc.add_page(
                printpdf::Mm(self.config.page_width_mm),
                printpdf::Mm(self.config.page_height_mm),
                "Layer 1"
            );
            
            let current_layer = doc.get_page(page_index).get_layer(layer_index);
            self.add_image_to_pdf_layer(&current_layer, image_path)?;
        }

        // 保存PDF - 使用BufWriter
        let file = std::fs::File::create(output)?;
        let mut buf_writer = BufWriter::new(file);
        doc.save(&mut buf_writer)?;
        
        log::info!("✅ PDF生成完成: {}", output.display());
        Ok(())
    }

    /// Get the current configuration
    pub fn config(&self) -> &PdfConfig {
        &self.config
    }

    /// Update the configuration
    pub fn set_config(&mut self, config: PdfConfig) {
        self.config = config;
    }

    /// Collect all image files from folder
    fn collect_image_files(&self, folder: &Path) -> Result<Vec<PathBuf>> {
        let image_extensions = ["jpg", "jpeg", "png", "gif", "bmp", "webp"];
        let mut image_files = Vec::new();

        for entry in std::fs::read_dir(folder)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                if let Some(extension) = entry.path().extension() {
                    if let Some(ext_str) = extension.to_str() {
                        if image_extensions.contains(&ext_str.to_lowercase().as_str()) {
                            image_files.push(entry.path());
                        }
                    }
                }
            }
        }

        Ok(image_files)
    }    /// Add an image to PDF page with automatic fitting
    fn add_image_to_pdf_layer(&self, current_layer: &PdfLayerReference, image_path: &Path) -> Result<()> {        // 读取并处理图片
        let img = ::image::open(image_path)?;
        let (img_width, img_height) = img.dimensions();
        
        // 转换为RGB8格式
        let rgb_img = img.to_rgb8();
        let raw_data = rgb_img.as_raw().clone();
        
        // 计算缩放和位置（居中显示，适配页面）
        let available_width_mm = self.config.page_width_mm - (2.0 * self.config.margin_mm);
        let available_height_mm = self.config.page_height_mm - (2.0 * self.config.margin_mm);
        
        // 根据DPI进行转换
        let pixel_to_mm = 25.4 / self.config.dpi;
        let img_width_mm = img_width as f32 * pixel_to_mm;
        let img_height_mm = img_height as f32 * pixel_to_mm;
        
        let scale_x = available_width_mm / img_width_mm;
        let scale_y = available_height_mm / img_height_mm;
        let scale = scale_x.min(scale_y);        let display_width_mm = img_width_mm * scale;
        let display_height_mm = img_height_mm * scale;        let x_mm = self.config.margin_mm + (available_width_mm - display_width_mm) / 2.0;
        // PDF坐标系统：(0,0)在左下角，Y轴向上为正
        // 计算正确的Y坐标 - 从页面底部开始向上
        let y_mm = self.config.margin_mm + (available_height_mm - display_height_mm) / 2.0;// 调试信息
        println!("  📊 图片原始尺寸: {}x{} px", img_width, img_height);
        println!("  📏 转换为mm: {:.1}x{:.1} mm", img_width_mm, img_height_mm);
        println!("  📐 可用空间: {:.1}x{:.1} mm", available_width_mm, available_height_mm);
        println!("  🔍 缩放比例: {:.3}", scale);
        println!("  📍 显示尺寸: {:.1}x{:.1} mm", display_width_mm, display_height_mm);
        println!("  🎯 位置: ({:.1}, {:.1}) mm", x_mm, y_mm);        // 创建图片对象
        let image_file = printpdf::Image::try_from(printpdf::ImageXObject {
            width: printpdf::Px(img_width as usize),
            height: printpdf::Px(img_height as usize),
            color_space: printpdf::ColorSpace::Rgb,
            bits_per_component: printpdf::ColorBits::Bit8,
            interpolate: true,
            image_data: raw_data,
            image_filter: None,
            clipping_bbox: None,
            smask: None,
        }).unwrap(); // This should never fail with valid inputs        // 添加图片到PDF - 恢复正确的缩放计算
        image_file.add_to_layer(
            current_layer.clone(),
            ImageTransform {
                translate_x: Some(printpdf::Mm(x_mm)),
                translate_y: Some(printpdf::Mm(y_mm)),
                scale_x: Some(scale),
                scale_y: Some(scale),
                rotate: None,
                dpi: Some(self.config.dpi),
            },
        );

        log::debug!("  图片添加成功: {}x{} -> {:.1}x{:.1}mm @ ({:.1}, {:.1})mm", 
            img_width, img_height, display_width_mm, display_height_mm, x_mm, y_mm);

        Ok(())
    }
}
