use log::{trace, debug};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{ status::Response, error::EPOSError};

pub const ENDPOINT: &'static str = "/cgi-bin/epos/service.cgi"; 

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename = "s:Envelope")]
pub struct SoapWrapper {
    #[serde(rename = "@xmlns:s")]
    pub ns: String,
    #[serde(rename = "s:Body",  skip_serializing_if = "Option::is_none")]
    pub body: Option<EposPrint>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename = "s:Envelope")]
pub struct SoapRespWrapper {
    #[serde(rename = "@xmlns:s")]
    pub ns: String,
    #[serde(rename = "$value")]
    pub body: ResponseBody
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseBody {
    #[serde(rename = "response")]
    pub response: Response
}



#[derive(Deserialize, Serialize, Debug)]
pub struct EposPrint {
    #[serde(rename = "@xmlns")]
    pub ns: String,

    #[serde(rename = "$value")]
    pub body: EnumBody
}
    
#[derive(Deserialize, Serialize, Debug)]
pub enum EnumBody {
    #[serde(rename = "epos-print")]
    NoPage {
        #[serde(rename = "$value")]
        body: String
    },
    #[serde(rename = "epos-print")]
    Page {
        #[serde(rename = "page")]
        body: PageWrapper
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PageWrapper {
    #[serde(rename = "$value")]
    pub body: String
}

pub async fn send(body: EnumBody, devid: &str, timeout: i32, endpoint: &Url) -> Result<(), EPOSError> {
    let full_request = SoapWrapper{
        ns: String::from("http://schemas.xmlsoap.org/soap/envelope/"),
        body: Some( EposPrint{
            ns: String::from("http://www.epson-pos.com/schemas/2011/03/epos-print"), 
            body: body
        }),
    };

    let output = quick_xml::se::to_string(&full_request)?;
    // we do this because we store the `self.build` as a string,  and the serialize option will escape the inner body
    let fixed = quick_xml::escape::unescape(&output).unwrap();
    trace!("Got complete XML: {}", fixed);


    let client = reqwest::Client::new();
    let params = [("devid", devid), ("timeout", &timeout.to_string())];
    let mut headers = HeaderMap::new();
    headers.insert(reqwest::header::CONTENT_TYPE,  "text/xml; charset=utf-8".parse()?);
    headers.insert(reqwest::header::IF_MODIFIED_SINCE, "Thu, 01 Jan 1970 00:00:00 GMT".parse()?);
    let builder = client.post(endpoint.clone()).query(&params).headers(headers).body(fixed.to_string());

    let resp = builder.send().await?.text().await?;
    let formatted_resp: SoapRespWrapper = quick_xml::de::from_str(&resp)?;
    debug!("Got raw response: {:?}", formatted_resp);
    if !formatted_resp.body.response.success {
        return Err(EPOSError::ResponseError { status: formatted_resp.body.response })
    }

    Ok(())
}