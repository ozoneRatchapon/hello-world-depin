use nostr::{key::Keys, SingleLetterTag, Timestamp, Kind};
use nostr_sdk::{Client, Filter, RelayPoolNotification};
use serde::Deserialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Deserialize)]
struct Message {
    greeting: String,
    solana_pubkey: String,
}

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

    // Initialize Solana testnet client
    let solana_client = RpcClient::new("https://api.devnet.solana.com".to_string());

    // Handle notifications asynchronously
    client
        .handle_notifications(|notification| async {
            match notification {
                RelayPoolNotification::Event { event, .. } => {
                    println!("Received {}", event.content);
                    // Parse the JSON content
                    if let Ok(message) = serde_json::from_str::<Message>(&event.content) {
                        // Request airdrop to the solana_pubkey
                        if let Ok(pubkey) = Pubkey::from_str(&message.solana_pubkey) {
                            match solana_client.request_airdrop(&pubkey, 1_000_000_000) {
                                Ok(signature) => {
                                    println!(
                                        "Airdrop requested for {}. Signature: {}",
                                        pubkey, signature
                                    );
                                    if let Ok(confirmed) =
                                        solana_client.confirm_transaction(&signature)
                                    {
                                        if confirmed {
                                            println!("Airdrop confirmed for {}", pubkey);
                                        }
                                    }
                                }
                                Err(e) => println!("Airdrop failed: {:?}", e),
                            }
                        } else {
                            println!("Invalid Solana pubkey: {}", message.solana_pubkey);
                        }
                    } else {
                        println!("Failed to parse message content");
                    }
                }
                _ => {} // Ignore other notification types
            }
            Ok(false) // Keep listening (false means don't stop)
        })
        .await?;

    Ok(())
}