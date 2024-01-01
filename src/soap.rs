use serde::{Deserialize, Serialize};

use crate::{builder::Body, status::Response};

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

    #[serde(rename = "epos-print")]
    pub body: EposBody
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EposBody {
    #[serde(rename = "$value")]
    pub list: Vec<Body>
}