//! Error types for PDF conversion operations

/// Result type alias for PDF conversion operations
pub type Result<T> = std::result::Result<T, PdfError>;

/// Error types that can occur during PDF conversion
#[derive(Debug, thiserror::Error)]
pub enum PdfError {
    /// IO error occurred
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Image processing error
    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),

    /// PDF generation error
    #[error("PDF error: {0}")]
    Pdf(#[from] printpdf::Error),

    /// No images found in the specified folder
    #[error("No images found in folder: {0}")]
    NoImagesFound(String),

    /// Invalid folder path
    #[error("Invalid folder path: {0}")]
    InvalidPath(String),

    /// Custom error with message
    #[error("{0}")]
    Custom(String),
}

impl PdfError {
    /// Create a custom error with a message
    pub fn custom<S: Into<String>>(message: S) -> Self {
        Self::Custom(message.into())
    }
}
