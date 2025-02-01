pub mod logging_table;
pub mod error;
pub mod utils;

use logging_table::LoggingTable;
use crate::error::ClouWatchViewerError;

use aws_sdk_cloudwatchlogs::Client;
use color_eyre::Result;

pub async fn handler(client: Client, log_group_name: &str) -> Result<Vec<LoggingTable>, ClouWatchViewerError> {
    let log_streams = client
        .describe_log_streams()
        .log_group_name(log_group_name)
        .send()
        .await?;

    let mut tasks = vec![];
    for log_stream in log_streams.log_streams() {
        if let Some(log_stream_name) = log_stream.log_stream_name() {
            let task = tokio::spawn(processs_log(
                client.clone(),
                log_group_name.to_string(),
                log_stream_name.to_string(),
                log_stream.creation_time,
                log_stream.first_event_timestamp,
                log_stream.last_event_timestamp,
                log_stream.last_ingestion_time,
                true,
            ));
            tasks.push(task);
        }
    }

    let mut records = vec![];
    for task in tasks {
        let logging_table = task.await??;
        records.extend(logging_table);
    }
    Ok(records)
}

async fn processs_log(
    client: Client,
    log_group_name: String,
    log_stream_name: String,
    log_creation_time: Option<i64>,
    first_event_timestamp: Option<i64>,
    last_event_timestamp: Option<i64>,
    last_ingestion_time: Option<i64>,
    start_from_head: bool,
) -> Result<Vec<LoggingTable>, ClouWatchViewerError> {
    let log_events = client
        .get_log_events()
        .log_group_name(log_group_name)
        .log_stream_name(&log_stream_name)
        .start_from_head(start_from_head)
        .send()
        .await?;

    let mut res = vec![];
    for event in log_events.events() {
        let logging_table = LoggingTable::new(
            log_stream_name.clone(),
            log_creation_time,
            first_event_timestamp,
            last_event_timestamp,
            last_ingestion_time,
            event.timestamp,
            event.message.clone(),
            event.ingestion_time,
        );
        res.push(logging_table);
    }

    Ok(res)
}
