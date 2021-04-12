use rusoto_kinesis::*;

use rusoto_core::Region;
use rusoto_core::RusotoError;

use futures::stream::StreamExt;

use tokio::time::{timeout};

#[tokio::main]
async fn main() -> Result<(), RusotoError<SubscribeToShardError>> {

    let client = KinesisClient::new(Region::UsWest2);

    let mut events: Vec<rusoto_kinesis::Record> = Vec::new();

    let mut event_stream = client.subscribe_to_shard(rusoto_kinesis::SubscribeToShardInput {
        consumer_arn: String::from(""),
        shard_id: String::from(""),
        starting_position: rusoto_kinesis::StartingPosition {
            sequence_number: None,
            timestamp: None,
            type_: String::from("TRIM_HORIZON"),
        },
    }).await?.event_stream;

    let read_events_future = async {
        while let Some(item) = event_stream.next().await {
            let payload = item.unwrap();
            println!("Got event from the event stream: {:?}", payload);

            match payload {
                rusoto_kinesis::SubscribeToShardEventStreamItem::SubscribeToShardEvent(e) => {
                    events.extend(e.records);
                },
                _ => {},
            }
        }
    };

    timeout(
        std::time::Duration::from_secs(10),
        read_events_future
    ).await.unwrap_err();

    Ok(())
}
