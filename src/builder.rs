//! The Main type for creating a complete receipt object.
use reqwest::{header::HeaderMap, IntoUrl};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{Builder, barcodes::{BarcodeType, HRI, SymbolType, ErrorCorrectionLevel}, 
formatters::{CutType, Font, FeedPos, Align, Lang}, 
soap::{SoapWrapper, EposPrint, EposBody, SoapRespWrapper}, error::{EPOSError, self}};

/// Body represents a single "item" in a ePOS receipt. A vector of `Body` objects represents a final receipt object.
#[derive(Deserialize, Serialize, Debug)]
pub enum Body {
    #[serde(rename = "text")]
    /// Convenience type, will produce the same output as `TextOps` without the clutter.
    /// Warning: If you're just printing a text value, the printer may fail to print unless you include a \n newline.
    Text {
        #[serde(rename = "$text")]
        text: String,
    },
    /// Produces a text line.
    /// Warning: If you're just printing a text value, the printer may fail to print unless you include a \n newline.
    #[serde(rename = "text")]
    TextOpts{
        #[serde(rename = "$text")]
        /// The text to print.
        text: String,
        #[serde(rename = "@font", skip_serializing_if = "Option::is_none")]
        /// Set the font.
        font: Option<Font>,
        #[serde(rename = "@smoothing", skip_serializing_if = "Option::is_none")]
        /// Set text smoothing.
        smoothing: Option<bool>,
        #[serde(rename = "@dw", skip_serializing_if = "Option::is_none")]
        /// Double Width. when specified with the `width` attr, the `width` will take precedence.
        double_width: Option<bool>,
        #[serde(rename = "@dh", skip_serializing_if = "Option::is_none")]
        /// Double Height. When specified with the `height` attr, the `height` will take precedence
        double_height: Option<bool>,
        #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
        /// Set text width. Must be a value between 1-8
        width: Option<u8>,
        #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
        /// Text Height. Must be a value between 1-8
        height: Option<u8>,
        #[serde(rename = "@ul", skip_serializing_if = "Option::is_none")]
        /// Set underline.
        underline: Option<bool>,
        #[serde(rename = "@em", skip_serializing_if = "Option::is_none")]
        /// Set emphasize.
        emph: Option<bool>,
        #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
        /// Set text color
        color: Option<bool>,
        #[serde(rename = "@lang", skip_serializing_if = "Option::is_none")]
        lang: Option<Lang>,
        #[serde(rename = "@align", skip_serializing_if = "Option::is_none")]
        align: Option<Align>

    },
    #[serde(rename = "cut")]
    /// Cut the paper.
    Cut{
        #[serde(rename = "@type")]
        cut_type: CutType
    },
    #[serde(rename ="feed")]
    /// Feed paper. At least one of the options for setting the length to feed must be set.
    Feed {
        #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
        /// Paper feed amount in dots
        unit: Option<u8>,
        #[serde(rename = "@line", skip_serializing_if = "Option::is_none")]
        /// Paper feed amount in lines
        line: Option<u8>,
        #[serde(rename = "@linespc", skip_serializing_if = "Option::is_none")]
        /// Per-line paper feed amount in dots
        linespc: Option<u8>,
        #[serde(rename = "@pos", skip_serializing_if = "Option::is_none")]
        /// Paper feed position of label paper/black mark paper 
        pos: Option<FeedPos>,
    },

    #[serde(rename ="image")]
    /// Print a bitmap raster image
    Image{
        #[serde(rename = "$text")]
        /// base-64 encoded raster image
        text: String,
        #[serde(rename = "@width")]
        width: i32,
        #[serde(rename = "@height")]
        height: i32
    },
    #[serde(rename ="barcode")]
    /// Print a 1D barcode
    Barcode {
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
    },
    #[serde(rename ="symbol")]
    /// Print a 2D barcode
    Symbol {
        #[serde(rename = "$text")]
        /// The text to print.
        text: String,
        #[serde(rename = "@type")]
        /// specifies the barcode type
        symbol_type: SymbolType,
        #[serde(rename = "@level", skip_serializing_if = "Option::is_none")]
        /// Set error correction level
        level: Option<ErrorCorrectionLevel>,
        #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
        /// specifies the width of the symbol in dots.
        /// Different symbol types have a different range of valid values:
        /// PDF417: 2-8
        /// QR: 1-16
        /// MaxiCode: ignored
        /// GS1Databar: 2-8
        /// Aztec: 2-16
        /// DataMatrix: 2-16
        width: Option<u8>,
        #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
        /// Only used by PDF417. Valid values are 2-8
        height: Option<u8>,
        #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
        /// Specifies the height. Only used by PDF417 (specifies the number of code words in each row)
        /// and Expanded Stacked GS1 DataBar (specifies the max width for the barcode, must be 106 or above)
        size: Option<u8>,
        #[serde(rename = "@align", skip_serializing_if = "Option::is_none")]
        /// Specifies code position
        align: Option<Align>,
        #[serde(rename = "@rotate", skip_serializing_if = "Option::is_none")]
        /// rotate the label
        rotate: Option<bool>
    }
}


/// Create a new ePOS-print handler
/// On most platforms, `device_id` will be `local_printer`
/// The `timeout` value is not an network timeout, but a timeout for the printer's internal parser logic.
pub fn new<U: IntoUrl, S: Into<String>>(url: U, timeout: i32, device_id: S) -> Result<Builder>{
    let endpoint: Url = url.into_url()?.join("/cgi-bin/epos/service.cgi")?;
    Ok(Builder { endpoint: endpoint, dev_id: device_id.into(), timeout: timeout })

}


impl Builder{
    /// Create an ePOS receipt
    pub async fn create(&self, body: Vec<Body>) -> Result<(), EPOSError> {

        let full_request = SoapWrapper{
            ns: String::from("http://schemas.xmlsoap.org/soap/envelope/"),
            body: Some( EposPrint{
                ns: String::from("http://www.epson-pos.com/schemas/2011/03/epos-print"), 
                body: EposBody{list: body}
            }),
        };

        let output = quick_xml::se::to_string(&full_request)?;
        println!("Got: {}", output);

        let client = reqwest::Client::new();
        let params = [("devid", &self.dev_id), ("timeout", &self.timeout.to_string())];
        let mut headers = HeaderMap::new();
        headers.insert(reqwest::header::CONTENT_TYPE,  "text/xml; charset=utf-8".parse()?);
        headers.insert(reqwest::header::IF_MODIFIED_SINCE, "Thu, 01 Jan 1970 00:00:00 GMT".parse()?);
        let builder = client.post(self.endpoint.clone()).query(&params).headers(headers).body(output);

        let resp = builder.send().await?.text().await?;
        let formatted_resp: SoapRespWrapper = quick_xml::de::from_str(&resp)?;
        if !formatted_resp.body.response.success {
            return Err(error::EPOSError::ResponseError { status: formatted_resp.body.response })
        }

        Ok(())
    }

}


