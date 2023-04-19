use anyhow::{anyhow, Context, Result};
use reqwest::{StatusCode, Url};
use tungstenite::Message;

/// Subscribes to the given pss topic using a websocket client.
/// Any non-empty messages that are received are immediately sent to the channel `output`
pub async fn subscribe_topic(
    endpoint: &str,
    topic: &str,
    output: tokio::sync::mpsc::Sender<Message>,
) -> Result<()> {
    let endpoint = format!("{endpoint}/pss/subscribe/{topic}");
    let (mut socket, _) =
        tungstenite::connect(Url::parse(&endpoint).with_context(|| "failed to parse endpoint")?)
            .with_context(|| "failed to connect")?;
    loop {
        match socket.read_message() {
            Ok(msg) => {
                if msg.is_empty() {
                    continue;
                }
                if let Err(err) = output.send(msg).await {
                    println!("failed to send message {err:#?}");
                }
            }
            Err(err) => {
                println!("failed to read message {err:#?}");
            }
        }
    }
}

/// Publishes `data` to `topic` with a target message prefix of `targets`
pub async fn publish_topic(
    client: reqwest::Client,
    endpoint: &str,
    topic: &str,
    targets: &str,
    recipient: &str,
    postage_batch_id: &str,
    data: &[u8],
) -> Result<()> {
    let endpoint = format!("{endpoint}/pss/send/{topic}/{targets}?recipient={recipient}");
    let req = client
        .post(endpoint)
        .header("swarm-postage-batch-id", postage_batch_id)
        .body(data.to_vec())
        .build()?;
    let res = client
        .execute(req)
        .await
        .with_context(|| "failed to execute request")?;
    if res.status().ne(&StatusCode::CREATED) {
        let reason = res.text().await.unwrap_or_default();
        return Err(anyhow!(
            "failed to publish message to topic {topic}: {reason}"
        ));
    }
    Ok(())
}
