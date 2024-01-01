//! epos-rs implements the ePOS API for creating and sending receipts to newer Epson printers
//! Complete documentation for the ePOS API can be found here: <https://files.support.epson.com/pdf/pos/bulk/tm-i_epos-print_um_en_revk.pdf>.
//! 
//! epos-rs handles all API details, providing an object-based API for creating receipt objects, and a `create` for handling the underlying XML and network request.
//! ```rust
//! use epos_rs::builder::{Body, new};
//! use epos_rs::barcodes::SymbolType;
//! use epos_rs::formatters::CutType;
//!  # tokio_test::block_on(async {
//! let barcode = Body::Symbol{ text: "This is a type 4 MaxiCode barcode".to_string(), 
//!     symbol_type: SymbolType::MaxiCodeMode4, level: None, 
//!     width: None, height: None, size: None, align: None, rotate: None };
//!
//! let feed = Body::Feed { unit: None, line: Some(5), linespc: None, pos: None };
//! let cut = Body::Cut { cut_type: CutType::Feed };
//! 
//! let handler = new("http://192.168.1.194", 10000, "local_printer").unwrap();
//! handler.create(vec![barcode, feed, cut]).await.unwrap();
//! # })
//! ```

use url::Url; 

pub mod builder;
mod soap;
pub mod barcodes;
pub mod formatters;
pub mod error;
pub mod status;

/// Builder manages the connection to the printer.
pub struct Builder {
    endpoint: Url,
    dev_id: String,
    timeout: i32,
}



#[cfg(test)]
mod tests {
    use crate::{builder::{Body, new}, formatters::Align};

    #[tokio::test]
    async fn test_se() {

        let ex1 = Body::TextOpts{text: String::from("I HATE XML\n\n"), underline:None, width: None, font: None, 
        smoothing: None, double_height: Some(true), double_width: Some(true), 
        height: None, emph: None, color: None, lang: None, align: Some(Align::Center)};
        let ex2 = Body::TextOpts{text: String::from("Writing drivers\n for\n recept printers\n sucks\n"), underline:None, width: None, font: None, 
        smoothing: None, double_height: Some(true), double_width: Some(true), 
        height: None, emph: None, color: None, lang: None, align: Some(Align::Center)};

        let barcode = Body::Symbol{ text: "HELP ME".to_string(), 
        symbol_type: crate::barcodes::SymbolType::MaxiCodeMode4, level: None, width: None, height: None, size: None, align: None, rotate: None };

        let feed = Body::Feed { unit: None, line: Some(5), linespc: None, pos: None };
        let cut = Body::Cut { cut_type: crate::formatters::CutType::Feed };

        let handler = new("http://192.168.1.194", 10000, "local_printer").unwrap();
        handler.create(vec![ex1, ex2, barcode, feed, cut]).await.unwrap();
    }

}