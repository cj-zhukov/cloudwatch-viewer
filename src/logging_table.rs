use std::sync::Arc;

use crate::error::ClouWatchViewerError;

use datafusion::{
    arrow::{
        array::{Int64Array, RecordBatch, StringArray},
        datatypes::{DataType, Field, Schema},
    },
    prelude::*,
};

// i64 expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC
#[derive(Debug)]
pub struct LoggingTable {
    pub log_stream_name: String,
    pub log_creation_time: Option<i64>,
    pub first_event_timestamp: Option<i64>,
    pub last_event_timestamp: Option<i64>,
    pub last_ingestion_time: Option<i64>,
    pub timestamp: Option<i64>,
    pub message: Option<String>,
    pub ingestion_time: Option<i64>,
}

impl LoggingTable {
    pub fn new(
        log_stream_name: String,
        log_creation_time: Option<i64>,
        first_event_timestamp: Option<i64>,
        last_event_timestamp: Option<i64>,
        last_ingestion_time: Option<i64>,
        timestamp: Option<i64>,
        message: Option<String>,
        ingestion_time: Option<i64>,
    ) -> Self {
        Self {
            log_stream_name,
            log_creation_time,
            first_event_timestamp,
            last_event_timestamp,
            last_ingestion_time,
            timestamp,
            message,
            ingestion_time,
        }
    }

    pub fn schema() -> Schema {
        Schema::new(vec![
            Field::new("log_stream_name", DataType::Utf8, false),
            Field::new("log_creation_time", DataType::Int64, true),
            Field::new("first_event_timestamp", DataType::Int64, true),
            Field::new("last_event_timestamp", DataType::Int64, true),
            Field::new("last_ingestion_time", DataType::Int64, true),
            Field::new("timestamp", DataType::Int64, true),
            Field::new("message", DataType::Utf8, true),
            Field::new("ingestion_time", DataType::Int64, true),
        ])
    }

    pub async fn to_df(ctx: &SessionContext, records: &Vec<Self>) -> Result<DataFrame, ClouWatchViewerError> {
        let schema = Self::schema();
        let mut log_stream_names = vec![];
        let mut log_creation_times = vec![];
        let mut first_event_timestamps = vec![];
        let mut last_event_timestamps = vec![];
        let mut last_ingestion_times = vec![];
        let mut timestamps = vec![];
        let mut messages = vec![];
        let mut ingestion_times = vec![];

        for record in records {
            log_stream_names.push(record.log_stream_name.clone());
            log_creation_times.push(record.log_creation_time);
            first_event_timestamps.push(record.first_event_timestamp);
            last_event_timestamps.push(record.last_event_timestamp);
            last_ingestion_times.push(record.last_ingestion_time);
            timestamps.push(record.timestamp);
            messages.push(record.message.clone());
            ingestion_times.push(record.ingestion_time);
        }

        let batch = RecordBatch::try_new(
            Arc::new(schema),
            vec![
                Arc::new(StringArray::from(log_stream_names)),
                Arc::new(Int64Array::from(log_creation_times)),
                Arc::new(Int64Array::from(first_event_timestamps)),
                Arc::new(Int64Array::from(last_event_timestamps)),
                Arc::new(Int64Array::from(last_ingestion_times)),
                Arc::new(Int64Array::from(timestamps)),
                Arc::new(StringArray::from(messages)),
                Arc::new(Int64Array::from(ingestion_times)),
            ],
        )?;

        Ok(ctx.read_batch(batch)?)
    }
}
