use std::error::Error;

use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct BeeConfig {
    pub upload: Option<UploadConfig>,
}

pub struct UploadConfig {
    pub stamp: String,
    pub pin: Option<bool>,
    // pub encrypt: Option<String>,
    pub tag: Option<String>,
    pub deferred: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwarmReference {
    pub ref_: String,
}

// download the data from the swarm using the bytes endpoint
pub async fn bytes_get(
    client: Client,
    base_uri: String,
    ref_: String,
) -> Result<(Vec<u8>, String), Box<dyn Error>> {
    let url = format!("{}/bytes/{}", base_uri, ref_);
    let res = client.get(&url).send().await?;

    // bubble up if there is an error
    if !res.status().is_success() {
        return Err(Box::new(res.error_for_status().unwrap_err()));
    }

    // read the response body
    let content_type = if let Some(content_type) = res.headers().get("Content-Type") {
        content_type.to_str()?.to_string()
    } else {
        "application/octet-stream".to_string()
    };

    let data = &res.bytes().await?;

    Ok((data.to_vec(), content_type))
}

// upload the data to the swarm using the bytes endpoint
// should return the reference from swarm
pub async fn bytes_post(
    client: Client,
    base_uri: String,
    data: Vec<u8>,
    config: &UploadConfig,
) -> Result<SwarmReference, Box<dyn Error>> {
    let mut headers = reqwest::header::HeaderMap::new();

    // process the config
    headers.insert("swarm-postage-batch-id", config.stamp.parse().unwrap());
    if let Some(pin) = config.pin && pin {
        headers.insert("swarm-pin", "true".parse().unwrap());
    }
    if let Some(tag) = &config.tag {
        headers.insert("swarm-tag", tag.parse().unwrap());
    }
    if let Some(deferred) = &config.deferred && !deferred {
        headers.insert("swarm-deferred", "false".parse().unwrap());
    }

    let res = client
        .post(format!("{}/bytes", base_uri))
        .body(data)
        .headers(headers)
        .send()
        .await?;
    let res = res.json::<SwarmReference>().await?;
    Ok(res)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
