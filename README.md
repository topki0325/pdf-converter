# PDF Converter

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Commercial Use](https://img.shields.io/badge/Commercial%20Use-Permitted-green.svg)](https://opensource.org/licenses/MIT)

A simple and efficient Rust library for converting images to PDF documents with support for multiple formats and commercial use.

## ✨ Features

- 🖼️ **Multi-format Support**: JPG, PNG, GIF, BMP, WebP
- 📄 **Single & Batch Conversion**: Convert individual files or entire folders
- 📐 **Smart A4 Fitting**: Automatic scaling with proper margins
- ⚙️ **Configurable**: Custom DPI, margins, page size, document title
- 🚀 **Performance Optimized**: Memory-efficient processing
- 💼 **Commercial Friendly**: MIT/Apache-2.0 dual license
- 🛠️ **Production Ready**: Comprehensive error handling and testing

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
pdf-converter = "0.1.0"
```

### Basic Usage

```rust
use pdf_converter::PdfConverter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let converter = PdfConverter::new();
    
    // Convert a single image
    converter.convert_image_to_pdf("image.jpg", "output.pdf")?;
    
    // Convert entire folder to single PDF
    converter.convert_folder_to_pdf("images/", "batch_output.pdf")?;
    
    Ok(())
}
```

### Advanced Configuration

```rust
use pdf_converter::{PdfConverter, PdfConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = PdfConfig {
        dpi: 300.0,
        margin_mm: 20.0,
        title: Some("My Document".to_string()),
    };
    
    let converter = PdfConverter::with_config(config);
    converter.convert_folder_to_pdf("images/", "high_quality.pdf")?;
    
    Ok(())
}
```

## 📊 Performance

- **Single image**: ~1 second
- **Batch (4 images)**: ~6 seconds  
- **Memory**: Optimized for large files
- **Quality**: 100-300 DPI support

## 🧪 Testing

Run the comprehensive test suite:

```bash
cargo run --bin comprehensive_test
```

Or quick validation:

```bash
cargo run --bin quick_test
```

## 📁 Project Structure

```
src/
├── lib.rs          # Public API
├── converter.rs    # Core conversion logic
└── error.rs        # Error handling
examples/
├── basic_usage.rs  # Simple example
└── batch_convert.rs # Batch processing
```

## 💼 Commercial Use

This project is **100% commercial-friendly**:

- ✅ **Use in proprietary software**
- ✅ **Sell products containing this code**
- ✅ **No copyleft restrictions**
- ✅ **Modify and redistribute**
- ✅ **Patent protection** (Apache-2.0)

Perfect for:
- Commercial applications
- SaaS products
- Enterprise software
- Consulting projects
- Proprietary systems

## 📄 License

Dual licensed under your choice of:

- **MIT License** - Simple and permissive
- **Apache License 2.0** - Includes patent protection

See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.

## 👤 Author

**topki**  
📧 Email: 259901434@qq.com

## 🤝 Contributing

Contributions are welcome! This project is designed for easy contribution with:

- Clear module separation
- Comprehensive error handling  
- Extensive test coverage
- Good documentation

## 🔗 Links

- [Documentation](https://docs.rs/pdf-converter)
- [Repository](https://github.com/topki/pdf-converter)
- [Issues](https://github.com/topki/pdf-converter/issues)

---

**Made with ❤️ by topki** • **Commercial use permitted** • **Production ready**
