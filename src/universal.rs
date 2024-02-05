//! Types that are available in normal mode and page mode.
use serde::{Deserialize, Serialize};

use crate::{formatters::{Font, Lang, Align, FeedPos}, page::PageItem, barcodes::{BarcodeType, HRI, SymbolType, ErrorCorrectionLevel}, normal::NormalItem};

/// Produces a text line.
/// Warning: If you're just printing a text value, the printer may fail to print unless you include a \n newline.
#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename = "text")]
pub struct Text {
    #[serde(rename = "$text")]
    /// The text to print.
    pub text: String,
    #[serde(rename = "@font", skip_serializing_if = "Option::is_none")]
    /// Set the font.
    pub font: Option<Font>,
    #[serde(rename = "@smoothing", skip_serializing_if = "Option::is_none")]
    /// Set text smoothing.
    pub smoothing: Option<bool>,
    #[serde(rename = "@dw", skip_serializing_if = "Option::is_none")]
    /// Double Width. when specified with the `width` attr, the `width` will take precedence.
    pub double_width: Option<bool>,
    #[serde(rename = "@dh", skip_serializing_if = "Option::is_none")]
    /// Double Height. When specified with the `height` attr, the `height` will take precedence
    pub double_height: Option<bool>,
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    /// Set text width. Must be a value between 1-8
    pub width: Option<u8>,
    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    /// Text Height. Must be a value between 1-8
    pub height: Option<u8>,
    #[serde(rename = "@ul", skip_serializing_if = "Option::is_none")]
    /// Set underline.
    pub underline: Option<bool>,
    #[serde(rename = "@em", skip_serializing_if = "Option::is_none")]
    /// Set emphasize.
    pub emph: Option<bool>,
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    /// Set text color
    pub color: Option<bool>,
    #[serde(rename = "@lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<Lang>,
    #[serde(rename = "@align", skip_serializing_if = "Option::is_none")]
    pub align: Option<Align>
}

impl PageItem for Text {}
impl NormalItem for Text{}


#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename ="feed")]
/// Feed paper. At least one of the options for setting the length to feed must be set.
/// Keep in mind that in page mode
pub struct Feed {
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    /// Paper feed amount in dots
    pub unit: Option<u8>,
    #[serde(rename = "@line", skip_serializing_if = "Option::is_none")]
    /// Paper feed amount in lines
    pub line: Option<u8>,
    #[serde(rename = "@linespc", skip_serializing_if = "Option::is_none")]
    /// Per-line paper feed amount in dots
    pub linespc: Option<u8>,
    #[serde(rename = "@pos", skip_serializing_if = "Option::is_none")]
    /// Paper feed position of label paper/black mark paper 
    pub pos: Option<FeedPos>,
}

impl PageItem for Feed{}
impl NormalItem for Feed{}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename ="barcode")]
pub struct Barcode {
    #[serde(rename = "$text")]
    /// The text to print.
    text: String,
    #[serde(rename = "@type")]
    /// specifies the barcode type
    barcode_type: BarcodeType,
    #[serde(rename = "@hri", skip_serializing_if = "Option::is_none")]
    /// Human Readable Interpretation settings
    hri: Option<HRI>,
    #[serde(rename = "@font", skip_serializing_if = "Option::is_none")]
    font: Option<Font>,
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    /// Specifies the barcode width. Must be a value from 2-6
    width: Option<u8>,
    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    /// Specifies barcode height
    height: Option<u8>,
    #[serde(rename = "@align", skip_serializing_if = "Option::is_none")]
    /// Specifies print position
    align: Option<Align>,
    #[serde(rename = "@rotate", skip_serializing_if = "Option::is_none")]
    /// Rotate the label
    rotate: Option<bool>
}

impl PageItem for Barcode {}
impl NormalItem for Barcode {}

/// Print a 2D barcode
#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename ="symbol")]
pub struct Symbol {
    #[serde(rename = "$text")]
    /// The text to print.
    pub text: String,
    #[serde(rename = "@type")]
    /// specifies the barcode type
    pub symbol_type: SymbolType,
    #[serde(rename = "@level", skip_serializing_if = "Option::is_none")]
    /// Set error correction level
    pub level: Option<ErrorCorrectionLevel>,
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    /// specifies the width of the symbol in dots.
    /// Different symbol types have a different range of valid values:
    /// PDF417: 2-8
    /// QR: 1-16
    /// MaxiCode: ignored
    /// GS1Databar: 2-8
    /// Aztec: 2-16
    /// DataMatrix: 2-16
    pub width: Option<u8>,
    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    /// Only used by PDF417. Valid values are 2-8
    pub height: Option<u8>,
    #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
    /// Specifies the height. Only used by PDF417 (specifies the number of code words in each row)
    /// and Expanded Stacked GS1 DataBar (specifies the max width for the barcode, must be 106 or above)
    pub size: Option<u8>,
    #[serde(rename = "@align", skip_serializing_if = "Option::is_none")]
    /// Specifies code position
    pub align: Option<Align>,
    #[serde(rename = "@rotate", skip_serializing_if = "Option::is_none")]
    /// rotate the label
    pub rotate: Option<bool>
}

impl PageItem for Symbol {}
impl NormalItem for Symbol {}

/// Print a bitmap raster image
#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename ="image")]
pub struct Image {
    #[serde(rename = "$text")]
    /// base-64 encoded raster image
    text: String,
    #[serde(rename = "@width")]
    width: i32,
    #[serde(rename = "@height")]
    height: i32
}

impl PageItem for Image {}
impl NormalItem for Image {}


#[cfg(test)]
mod tests {
    use crate::formatters::Font;

    use super::Text;

    #[test]
    fn test_text() {
        let test = Text{text: String::from("test"), 
            font: Some(Font::FontA), 
            smoothing: Some(true),
            double_width: Some(true),
            double_height: Some(true),
            width: Some(20),
            height: Some(20),
            underline: Some(true),
            emph: Some(true),
            color: Some(true),
            lang: Some(crate::formatters::Lang::En),
            align: Some(crate::formatters::Align::Center)
        };
        let out = quick_xml::se::to_string(&test).unwrap();
        assert_eq!(out, String::from(r#"<text font="font_a" smoothing="true" dw="true" dh="true" width="20" height="20" ul="true" em="true" color="true" lang="en" align="center">test</text>"#))
    }
}