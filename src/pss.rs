use anyhow::{Result, anyhow, Context};
use reqwest::Url;
pub async fn subscribe_topic(
    endpoint: &str,
    topic: &str,
) -> Result<()> {
    let endpoint = format!("{endpoint}/pss/subscribe/{topic}");
    println!("endpoint {endpoint}");
    let (mut socket, _) = tungstenite::connect(Url::parse(&endpoint).with_context(|| "failed to parse endpoint")?).with_context(|| "failed to connect")?;
    loop {
        match socket.read_message() {
            Ok(msg) => {
                if msg.is_empty() {
                    continue;
                }
                println!("receive message {msg:#?}");
            }
            Err(err) => {
                println!("failed to read message {err:#?}");
            }
        }
    }
}
pub async fn publish_topic(
    client: reqwest::Client,
    endpoint: &str,
    topic: &str,
    targets: &str,
    recipient: &str,
    postage_batch_id: &str,
    data: &[u8]
) -> Result<()> {
    let endpoint = format!("{endpoint}/pss/send/{topic}/{targets}?recipient={recipient}");
    let req = client.post(endpoint).header("swarm-postage-batch-id", postage_batch_id).body(data.to_vec()).build()?;
    let res = client.execute(req).await.with_context(|| "failed to execute request")?;
    println!("{res:#?}");
    Ok(())
}