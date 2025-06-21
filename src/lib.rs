//! # PDF Converter
//! 
//! A simple library for converting JPG and other image formats to PDF documents.
//! 
//! ## Features
//! 
//! - Convert single images to PDF
//! - Batch convert multiple images from a folder to a single PDF
//! - Support for JPG, PNG, GIF, BMP, WebP formats
//! - Automatic A4 page fitting with proper scaling
//! - Configurable margins and page settings
//! 
//! ## Example
//! 
//! ```rust
//! use pdf_converter::PdfConverter;
//! 
//! let converter = PdfConverter::new();
//! converter.convert_folder_to_pdf("images/", "output.pdf")?;
//! ```

pub mod converter;
pub mod error;

pub use converter::{PdfConverter, PdfConfig};
pub use error::{PdfError, Result};

/// Default A4 page width in millimeters
pub const A4_WIDTH_MM: f32 = 210.0;

/// Default A4 page height in millimeters
pub const A4_HEIGHT_MM: f32 = 297.0;

/// Default page margin in millimeters
pub const DEFAULT_MARGIN_MM: f32 = 20.0;

/// Default DPI for image conversion
pub const DEFAULT_DPI: f32 = 300.0;
