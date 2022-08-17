use reqwest::Client;
use serde::{Deserialize, Serialize};


pub struct SwarmUploadConfig {
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


// upload the data to the swarm using the bytes endpoint
// should return the reference from swarm
pub async fn bytes_post(client: Client, base_uri: String, data: Vec<u8>, config: SwarmUploadConfig) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut headers = reqwest::header::HeaderMap::new();

    // process the config
    headers.insert("swarm-postage-batch-id", config.stamp.parse().unwrap());
    if let Some(pin) = config.pin && pin {
        headers.insert("swarm-pin", "true".parse().unwrap());
    }
    if let Some(tag) = config.tag {
        headers.insert("swarm-tag", tag.parse().unwrap());
    }
    if let Some(deferred) = config.deferred && !deferred {
        headers.insert("swarm-deferred", "false".parse().unwrap());
    }

    let res = client
        .post(format!("{}/bytes", base_uri))
        .body(data)
        .headers(headers)
        .send()
        .await?;
    let res = res.json::<SwarmReference>().await?;
    Ok(res.ref_.as_bytes().to_vec())
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
