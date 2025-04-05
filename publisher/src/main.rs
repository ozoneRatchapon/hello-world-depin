use nostr::{key::Keys, Kind};
use nostr_sdk::{Client, EventBuilder, Options, Tag};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Nostr client
    let keys: Keys = Keys::generate(); // Generate a keypair for the publisher
    let client_opts = Options::default();

    let client = Client::builder()
        .signer(keys.clone())
        .opts(client_opts)
        .build();

    client.add_relay("wss://dev-relay.dephy.dev").await?;
    client.connect().await;

    // Create and send a "Hello World" event
    let event = EventBuilder::new(Kind::Custom(1573), "Hello World").tags([
        Tag::parse(["s".to_string(), "hello_session".to_string()]).unwrap(),
        Tag::parse(["p".to_string(), "receiver_pubkey".to_string()]).unwrap(),
    ]);

    client.send_event_builder(event).await?;
    println!("Published 'Hello World' event to wss://dev-relay.dephy.dev");

    Ok(())
}