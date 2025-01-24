use cloudwatch_viewer::utils::constants::{LOGGING_TABLE_NAME, LOG_GROUP_NAME_SECRET, REGION};
use cloudwatch_viewer::utils::datafusion::{df_plan_to_table, write_df_to_file};
use cloudwatch_viewer::LoggingTable;
use cloudwatch_viewer::{handler, utils::aws::get_aws_client};

use color_eyre::Result;
use datafusion::prelude::SessionContext;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let client = get_aws_client(REGION).await;
    let ctx = SessionContext::new();
    let records = handler(client, &LOG_GROUP_NAME_SECRET).await?;
    let df = LoggingTable::to_df(&ctx, &records).await?;
    df_plan_to_table(&ctx, df.logical_plan().clone(), LOGGING_TABLE_NAME).await?;
    // let res = ctx.sql(&format!("select * from {LOGGING_TABLE_NAME}")).await?;
    // write_df_to_file(res, "logging.parquet").await?;
    let res = ctx.sql(&format!("select * from {LOGGING_TABLE_NAME} where message like '%rs%' order by timestamp limit 10")).await?;
    res.show().await?;

    Ok(())
}
