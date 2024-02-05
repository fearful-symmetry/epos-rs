# epos-rs

`epos-rs` is a rust driver for Epson receipt printers that implements the [EPOS Print](https://files.support.epson.com/pdf/pos/bulk/epos-print_xml_um_en_revi.pdf) API.

```rust
use epos_rs::Builder;
use epos_rs::universal::{Symbol, Text};
use epos_rs::normal::Cut;
use epos_rs::barcodes::SymbolType;
use epos_rs::formatters::CutType;
 
// normal() returns a handler for "normal" mode, which prints commands in-order.
// page() will return a handler for page mode, which prints a page in a specified print area.
let mut handler = Builder::new(10000, "local_printer", "http://192.168.1.194").unwrap().normal();
 
// Add a 2D MaxiCode barcode.
handler.add(Symbol{text: "This is a type 4 MaxiCode barcode".to_string(), 
    symbol_type: SymbolType::MaxiCodeMode4,  ..Default::default()}).unwrap();
// Add some text
handler.add(Text{text: String::from("This is some text\n\n"), ..Default::default()}).unwrap();
// feed and cut
handler.add(Cut{cut_type: CutType::Feed}).unwrap();
// Send to printer
handler.print().await.unwrap();

```

## Features

`epos-rs` is currently not feature-complete with the ePOS API. Currently missing features:

- `vline-begin` for normal mode
- `vline-end` for normal mode
- Exported `status` API commands
- `pulse` for normal mode
- `sound` for normal mode
- `command` XML element
- `layout` XML element
- `recovery` XML element
- `reset` XML element
