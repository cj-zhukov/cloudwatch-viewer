use std::io::Error as IoError;

use aws_sdk_cloudwatchlogs::operation::describe_log_streams::DescribeLogStreamsError;
use aws_sdk_cloudwatchlogs::operation::get_log_events::GetLogEventsError;
use aws_sdk_cloudwatchlogs::error::SdkError;
use color_eyre::eyre::Report;
use datafusion::arrow::error::ArrowError;
use datafusion::error::DataFusionError;
use datafusion::parquet::errors::ParquetError;
use tokio::task::JoinError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClouWatchViewerError {
    #[error("Arrow error")]
    ArrowError(#[from] ArrowError),

    #[error("AWS DescribeLogStreams error")]
    DescribeLogStreamsError(#[from] SdkError<DescribeLogStreamsError>),

    #[error("AWS GetLogEventsError error")]
    GetLogEventsError(#[from] SdkError<GetLogEventsError>),

    #[error("DataFusion error")]
    DataFusionError(#[from] DataFusionError),

    #[error("IO error")]
    IoError(#[from] IoError),

    #[error("Parquet error")]
    ParquetError(#[from] ParquetError),

    #[error("TokioJoin error")]
    TokioJoinError(#[from] JoinError),
    
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}
