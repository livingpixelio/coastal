use iroh::endpoint::presets;
use iroh::{protocol::Router, Endpoint};
use iroh_gossip::{api::Event, Gossip, TopicId};
use iroh_tickets::endpoint::EndpointTicket;
use n0_future::StreamExt;
use sha2::{Digest, Sha256};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub async fn create_share(url: &str) -> Result<String, String> {
    // create an iroh endpoint that includes the standard discovery mechanisms
    // we've built at number0
    let endpoint = Endpoint::bind(presets::N0)
        .await
        .map_err(|e| e.to_string())?;

    // get the ticket before moving endpoint into the background task
    let ticket = EndpointTicket::new(endpoint.addr());

    let url = url.to_string();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = create_relay_from_endpoint(endpoint, &url).await {
            eprintln!("relay error: {e}");
        }
    });

    Ok(format!("You are trying to share: {}", ticket))
}

fn url2topicid(url: &str) -> TopicId {
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    let topic_id_bytes = hasher.finalize();
    TopicId::from_bytes(topic_id_bytes.into())
}

async fn create_relay_from_endpoint(endpoint: Endpoint, url: &str) -> Result<(), String> {
    // build gossip protocol
    let gossip = Gossip::builder().spawn(endpoint.clone());

    // setup router
    let router = Router::builder(endpoint)
        .accept(iroh_gossip::ALPN, gossip.clone())
        .spawn();

    // subscribe to the topic derived from the URL
    let (_sender, mut receiver) = gossip
        .subscribe(url2topicid(url), vec![])
        .await
        .map_err(|e| e.to_string())?
        .split();

    // you might want to wait until you joined at least one other peer:
    receiver.joined().await.map_err(|e| e.to_string())?;

    // read messages from others
    while let Some(event) = receiver.next().await {
        match event.map_err(|e| e.to_string())? {
            Event::Received(message) => {
                println!(
                    "received a message: {:?}",
                    std::str::from_utf8(&message.content)
                );
            }
            _ => {}
        }
    }

    // clean shutdown makes sure that other peers are notified that you went offline
    router.shutdown().await.map_err(|e| e.to_string())?;
    Ok(())
}
