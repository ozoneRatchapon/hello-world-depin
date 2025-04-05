use nostr::{key::Keys, Kind};
use nostr_sdk::{Client, EventBuilder, Options, Tag};
use serde::Serialize;
use solana_sdk::{signature::Keypair, signer::Signer};

#[derive(Serialize)]
struct Message {
    greeting: String,
    solana_pubkey: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate Solana keypair
    let solana_keypair = Keypair::new();
    let solana_pubkey = solana_keypair.pubkey().to_string();

    // Initialize the Nostr client
    let keys: Keys = Keys::generate(); // Generate a keypair for the publisher
    let client_opts = Options::default();

    let client = Client::builder()
        .signer(keys.clone())
        .opts(client_opts)
        .build();

    client.add_relay("wss://dev-relay.dephy.dev").await?;
    client.connect().await;

    // Create message struct
    let message = Message {
        greeting: "Hello World".to_string(),
        solana_pubkey: solana_pubkey.clone(),
    };
    
    // Create and send a "Hello World" event
    let event = EventBuilder::new(Kind::Custom(1573), serde_json::to_string(&message)?).tags([
        Tag::parse(["s".to_string(), "hello_session".to_string()])?,
        Tag::parse(["p".to_string(), "receiver_pubkey".to_string()])?,
    ]);

    client.send_event_builder(event).await?;
    println!("Published 'Hello World' event to wss://dev-relay.dephy.dev");
    println!("Airdrop request for {}", solana_pubkey);

    Ok(())
}