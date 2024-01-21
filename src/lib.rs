//! epos-rs implements the ePOS API for creating and sending receipts to newer Epson printers that support the ePOS network API.
//! 
//! Complete documentation for the ePOS API can be found here: <https://files.support.epson.com/pdf/pos/bulk/epos-print_xml_um_en_revi.pdf>.
//! 
//! epos-rs handles all API details, providing an object-based interface for creating receipt objects, and a `print` method for handling the underlying XML and network request.
//! ```rust
//! # tokio_test::block_on(async {
//! use epos_rs::new;
//! use epos_rs::universal::{Symbol, Text};
//! use epos_rs::normal::Cut;
//! use epos_rs::barcodes::SymbolType;
//! use epos_rs::formatters::CutType;
//! 
//! // normal() returns a handler for "normal" mode, which prints commands in-order.
//! // page() will return a handler for page mode, which prints a page in a specified print area.
//! let mut handler = new(10000, "local_printer", "http://192.168.1.194").unwrap().normal();
//! 
//! // Add a 2D MaxiCode barcode.
//! handler.add(Symbol{text: "This is a type 4 MaxiCode barcode".to_string(), 
//!     symbol_type: SymbolType::MaxiCodeMode4,  ..Default::default()}).unwrap();
//! // Add some text
//! handler.add(Text{text: String::from("This is some text\n\n"), ..Default::default()}).unwrap();
//! // feed and cut
//! handler.add(Cut{cut_type: CutType::Feed}).unwrap();
//! handler.print().await.unwrap();
//! 

//! # })
//! ```

use std::fmt::Display;

use error::EPOSError;
use normal::NormalItem;
use page::PageItem;
use quick_xml::DeError;
use reqwest::IntoUrl;
use soap::{ENDPOINT, EnumBody, PageWrapper};
use url::Url; 


mod soap;

pub mod barcodes;
pub mod formatters;
pub mod error;
pub mod status;
pub mod page;
pub mod normal;
pub mod universal;

/// Builder manages the connection to the printer.
#[derive(Clone, Debug)]
pub struct Builder {
    endpoint: Url,
    dev_id: String,
    timeout: i32,
}

/// Create a new printer connection. To use this connection to print, call either `builder.page()` or `builder.normal()`.
/// On most printers, the default device ID is `"local_printer"`.
/// The `timeout` is not a network timeout, but serves as a device-side parser timeout. On most systems, a reasonable timeout is ~10000.
pub fn new<U: IntoUrl>(timeout: i32, dev_id: &str, endpoint: U) -> Result<Builder, EPOSError> {
    Ok( Builder{
        timeout: timeout,
        dev_id: dev_id.to_string(),
        endpoint: endpoint.into_url()?.join(ENDPOINT)?
    })
}


impl Builder {
    /// create a new builder object for writing in page mode (a formatted area of a set size).
    pub fn page(&self) -> PageBuilder {
        PageBuilder{
            build: Vec::new(),
            timeout: self.timeout,
            dev_id: self.dev_id.clone(),
            endpoint: self.endpoint.clone()
        }
    }

    /// create a new builder object for writing in normal mode (commands are printed one line at a time).
    pub fn normal(&self) -> NormalBuilder {
        NormalBuilder {
            build: Vec::new(),
            timeout: self.timeout,
            dev_id: self.dev_id.clone(),
            endpoint: self.endpoint.clone()
        }
    }
}

/// manage and track a print job in page mode.
 #[derive(Clone, Debug)]
pub struct PageBuilder {
    build: Vec<String>,
    timeout: i32,
    dev_id: String,
    endpoint: Url
}

impl Display for PageBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.build)
    }
}

impl PageBuilder {

    /// Add a command to the page
    pub fn add <I: PageItem> (&mut self, item: I) -> Result<(), DeError> {
        let output = quick_xml::se::to_string(&item)?;
        self.build.push(output);
        Ok(())
    }

    /// print a page
    pub async fn print(&mut self) -> Result<(), EPOSError> {
        let final_body = EnumBody::Page { body: PageWrapper{body: self.build.join("\n")}}; 
        soap::send(final_body, &self.dev_id, self.timeout, &self.endpoint).await?;

        Ok(())
    }
}

/// Manage and track a print job in normal mode
pub struct NormalBuilder {
    build: Vec<String>,
    timeout: i32,
    dev_id: String,
    endpoint: Url
}

impl Display for NormalBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.build)
    }
}

impl NormalBuilder {
    /// Add a command
    pub fn add <I: NormalItem> (&mut self, item: I) -> Result<(), DeError> {
        let output = quick_xml::se::to_string(&item)?;
        self.build.push(output);
        Ok(())
    }

    /// print the document
    pub async fn print(&mut self) -> Result<(), EPOSError> {
        let final_body = EnumBody::NoPage { body:  self.build.join("\n")}; 
        soap::send(final_body, &self.dev_id, self.timeout, &self.endpoint).await?;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::{formatters::Align, page, new, universal::{Text, Symbol, Feed}, barcodes::SymbolType, normal::{Cut, Hline}};

    #[tokio::test]
    async fn test_normal() {

        let mut handler = new(10000, "local_printer", "http://192.168.1.194").unwrap().normal();

        handler.add(Text{text: String::from("I HATE XML\n\n"), double_height: Some(true), 
        double_width: Some(true), align: Some(Align::Center), ..Default::default()}).unwrap();

        handler.add(Text{text: "Writing drivers\n for\n recept printers\n sucks\n".to_string(), 
        double_height: Some(false), double_width: Some(false), align: Some(Align::Center), ..Default::default()}).unwrap();

        handler.add(Hline{x1: 100, x2: 200, style: Some(crate::formatters::Style::ThinDouble)}).unwrap();
        handler.add(Hline{x1: 400, x2: 500, style: Some(crate::formatters::Style::ThinDouble)}).unwrap();

        handler.add(Symbol{text: "HELP ME".to_string(), symbol_type: SymbolType::MaxiCodeMode4, ..Default::default()}).unwrap();
        handler.add(Feed{line: Some(5), ..Default::default()}).unwrap();
        handler.add(Cut{cut_type: crate::formatters::CutType::Feed}).unwrap();

        handler.print().await.unwrap();
    }

    #[tokio::test]
    async fn test_page() {
        let area = page::Area { x: 0, y: 0, width: 500, height: 500 };
        let ex1 = Text{text: String::from("\nI HATE XML\n\n"), underline:None, width: None, font: None, 
        smoothing: None, double_height: Some(false), double_width: Some(false), 
        height: None, emph: None, color: None, lang: None, align: Some(Align::Center)};

        let rect = page::Rectangle{ x1: 0, y1: 0, x2: 200, y2: 100, style: None };

        let feed = Feed { unit: None, line: Some(200), linespc: None, pos: None };

        let mut handler = new(10000, "local_printer",  "http://192.168.1.194").unwrap().page();
        handler.add(area).unwrap();
        handler.add(ex1).unwrap();
        handler.add(rect).unwrap();
        handler.add(feed).unwrap();

        handler.print().await.unwrap();
    }


}