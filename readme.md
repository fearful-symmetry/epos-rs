# epos-rs

`epos-rs` is a rust driver for Epson receipt printers that implement the [EPOS Print](https://files.support.epson.com/pdf/pos/bulk/epos-print_xml_um_en_revi.pdf) API

```rust
use epos_rs::builder::{Body, new};
use epos_rs::barcodes::SymbolType;
use epos_rs::formatters::CutType;

let barcode = Body::Symbol{ text: "This is a type 4 MaxiCode barcode".to_string(), 
    symbol_type: SymbolType::MaxiCodeMode4, level: None, 
    width: None, height: None, size: None, align: None, rotate: None };

let feed = Body::Feed { unit: None, line: Some(5), linespc: None, pos: None };
let cut = Body::Cut { cut_type: CutType::Feed };

let handler = new("http://192.168.1.194", 10000, "local_printer").unwrap();
handler.create(vec![barcode, feed, cut]).await.unwrap();

```

`epos-rs` is currently not feature-complete with the ePOS API, although it supports basic features such as:

- All text and text formatting options
- All 1D and 2D barcodes and & Barcode options
- Images
- Feeding and cutting