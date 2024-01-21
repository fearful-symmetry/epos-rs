use quick_xml::DeError;
use reqwest::IntoUrl;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{formatters::Style, error::EPOSError, soap::{EnumBody, PageWrapper, self, ENDPOINT}};

pub trait PageItem: Serialize {}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename ="area")]
pub struct Area {
        /// Start point for the print area
        #[serde(rename = "@x")]
        pub x: u16,
        /// End point for the print area
        #[serde(rename = "@y")]
        pub y: u16,
        /// Total Print area width
        #[serde(rename = "@width")]
        pub width: u16,
        /// Total print area height
        #[serde(rename = "@height")]
        pub height: u16,
}

impl PageItem for Area {}



#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename ="rectangle")]
pub struct Rectangle {
    /// Specifies the horizontal draw start position in units of dots.
    #[serde(rename = "@x1")]
    pub x1: u16,
    /// Specifies the vertical draw start position in units of dots.
    #[serde(rename = "@y1")]
    pub y1: u16,
    /// Specifies the horizontal draw end position in units of dots.
    #[serde(rename = "@x2")]
    pub x2:u16,
    /// Specifies the vertical draw end position in units of dots.
    #[serde(rename = "@y2")]
    pub y2: u16,
    #[serde(rename = "@style", skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>    
}

impl PageItem for Rectangle{}


pub struct PageBuilder {
    build: Vec<String>,
    timeout: i32,
    dev_id: String,
    endpoint: Url
}

pub fn new<U: IntoUrl>(timeout: i32, dev_id: String, endpoint: U) -> Result<PageBuilder, EPOSError> {
   Ok( PageBuilder{
        build: Vec::new(),
        timeout: timeout,
        dev_id: dev_id,
        endpoint: endpoint.into_url()?.join(ENDPOINT)?
    })
}

impl PageBuilder {

    pub fn add <I: PageItem> (&mut self, item: I) -> Result<(), DeError> {
        let output = quick_xml::se::to_string(&item)?;
        self.build.push(output);
        Ok(())
    }

    pub async fn print(&mut self) -> Result<(), EPOSError> {

        let final_body = EnumBody::Page { body: PageWrapper{body: self.build.join("\n")}}; 
        soap::send(final_body, &self.dev_id, self.timeout, &self.endpoint).await?;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use url::Url;

    use crate::{universal, formatters::Align, soap::ENDPOINT, page::new};

    use super::{PageBuilder, Rectangle, Area};


    #[tokio::test]
    async fn print_test() {

        let mut handler = new(1000, "local_printer".to_string(), "http://192.168.1.194").unwrap();

        handler.add(Area{ x: 0, y: 0, width: 500, height: 500 }).unwrap();
        handler.add(universal::Text{text: String::from("\nI HATE XML\n\n"), align: Some(Align::Center), ..Default::default()}).unwrap();
        handler.add(Rectangle{ x1: 0, y1: 0, x2: 200, y2: 100, style: None }).unwrap();
        handler.add(universal::Feed { line: Some(200), ..Default::default()}).unwrap();

        handler.print().await.unwrap();
    }
}