use std::error;

use reqwest::Client;
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn error::Error + Send>>;

#[derive(Debug, Clone)]
pub struct BeeConfig {
    pub upload: Option<UploadConfig>,
}

#[derive(Debug, Clone)]
pub struct UploadConfig {
    pub stamp: String,
    pub pin: Option<bool>,
    // pub encrypt: Option<String>,
    pub tag: Option<u32>,
    pub deferred: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwarmReference {
    #[serde(rename(deserialize = "reference"))]
    pub ref_: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwarmTag {
    pub uid: u32,
    #[serde(rename(deserialize = "startedAt"))]
    pub started_at: String,
    pub total: u64,
    pub processed: u64,
    pub synced: u64,
}

// download the data from the swarm using the bytes endpoint
pub async fn bytes_get(
    client: Client,
    base_uri: String,
    ref_: String,
) -> Result<(Vec<u8>, String)> {
    let url = format!("{}/bytes/{}", base_uri, ref_);
    let res = client.get(&url).send().await;

    // bubble up if there was an error making the request
    let res = match res {
        Ok(res) => res,
        Err(e) => return Err(Box::new(e)),
    };

    // bubble up if the response was not successful
    if !res.status().is_success() {
        return Err(Box::new(res.error_for_status().unwrap_err()));
    }

    // get the content type or set default if not present
    let content_type = res
        .headers()
        .get("content-type")
        .map(|ct| ct.to_str().unwrap().to_string())
        .unwrap_or_else(|| "application/octet-stream".to_string());

    // get the data from the response
    let data = res.bytes().await;
    match data {
        Ok(data) => Ok((data.to_vec(), content_type)),
        Err(e) => Err(Box::new(e)),
    }
}

// create a new tag
pub async fn tag_post(
    client: &Client,
    base_uri: String,
) -> Result<SwarmTag> {
    let url = format!("{}/tags", base_uri);
    let res = client.post(&url).send().await;

    // bubble up if there was an error making the request
    let res = match res {
        Ok(res) => res,
        Err(e) => return Err(Box::new(e)),
    };

    // bubble up if the response was not successful
    if !res.status().is_success() {
        return Err(Box::new(res.error_for_status().unwrap_err()));
    }

    // get the data from the response
    let data = res.json::<SwarmTag>().await;
    match data {
        Ok(data) => Ok(data),
        Err(e) => Err(Box::new(e)),
    }
}

// get information on a tag
pub async fn get_tag(
    client: Client,
    base_uri: String,
    tag: u32,
) -> Result<SwarmTag> {
    let res = client
        .post(format!("{}/tags/{}", base_uri, tag))
        .send()
        .await;
    
    // bubble up if there is an error
    match res {
        Ok(res) => match res.status().is_success() {
            true => match res.json::<SwarmTag>().await {
                Ok(ref_) => Ok(ref_),
                Err(e) => Err(Box::new(e)),
            },
            false => Err(Box::new(res.error_for_status().unwrap_err())),
        },
        Err(e) => Err(Box::new(e)),
    }
}

// upload the data to the swarm using the bytes endpoint
// should return the reference from swarm
pub async fn bytes_post(
    client: Client,
    base_uri: String,
    data: Vec<u8>,
    config: &UploadConfig,
) -> Result<SwarmReference> {
    let mut headers = reqwest::header::HeaderMap::new();

    // process the config
    headers.insert("swarm-postage-batch-id", config.stamp.parse().unwrap());
    if let Some(pin) = config.pin && pin {
        headers.insert("swarm-pin", "true".parse().unwrap());
    }
    if let Some(tag) = &config.tag {
        headers.insert("swarm-tag", tag.to_string().parse().unwrap());
    }
    if let Some(deferred) = &config.deferred && !deferred {
        headers.insert("swarm-deferred", "false".parse().unwrap());
    }

    let res = client
        .post(format!("{}/bytes", base_uri))
        .body(data)
        .headers(headers)
        .send()
        .await;
    // bubble up if there is an error
    match res {
        Ok(res) => match res.status().is_success() {
            true => match res.json::<SwarmReference>().await {
                Ok(ref_) => Ok(ref_),
                Err(e) => Err(Box::new(e)),
            },
            false => Err(Box::new(res.error_for_status().unwrap_err())),
        },
        Err(e) => Err(Box::new(e)),
    }
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
