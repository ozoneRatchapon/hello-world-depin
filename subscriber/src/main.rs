use nostr::{key::Keys, Kind, SingleLetterTag, Timestamp};
use nostr_sdk::{Client, Filter, RelayPoolNotification};

const MENTION_TAG: SingleLetterTag = SingleLetterTag::lowercase(nostr::Alphabet::P);
const SESSION_TAG: SingleLetterTag = SingleLetterTag::lowercase(nostr::Alphabet::S);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Nostr client
    let keys: Keys = Keys::generate();
    let client_opts = nostr_sdk::Options::default();

    let client = Client::builder()
        .signer(keys.clone())
        .opts(client_opts)
        .build();

    client.add_relay("wss://dev-relay.dephy.dev").await?;
    client.connect().await;

    // Define a filter for "Hello World" events
    let filter = Filter::new()
        .kind(Kind::Custom(1573))
        .since(Timestamp::now())
        .custom_tag(SESSION_TAG, "hello_session")
        .custom_tag(MENTION_TAG, "receiver_pubkey");

    // Subscribe to the filter
    client.subscribe(filter, None).await?;

    println!("Subscribed events on wss://dev-relay.dephy.dev");

    // Handle notifications asynchronously
    client
        .handle_notifications(|notification| async {
            match notification {
                RelayPoolNotification::Event { event, .. } => {
                    println!("Received: {}", event.content);
                }
                _ => {} // Ignore other notification types
            }
            Ok(false) // Keep listening (false means don't stop)
        })
        .await?;

    Ok(())
}