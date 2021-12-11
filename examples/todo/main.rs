mod commands;

use anyhow::{Context, Result};
use clap::Parser;
use notion::ids::DatabaseId;
use notion::NotionApi;
use serde::{Deserialize, Serialize};

// From <https://docs.rs/clap/3.0.0-beta.2/clap/>
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Jake Swenson")]
struct Opts {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Configure what database this notion-todo example uses
    Config,
    /// List all todos
    List,
    /// Add a todo item to the notion database
    Add,
    /// Complete a todo item
    Check,
}

#[derive(Deserialize, Serialize)]
struct TodoConfig {
    api_token: Option<String>,
    task_database_id: Option<DatabaseId>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    // https://docs.rs/config/0.11.0/config/
    let config = config::Config::default()
        .with_merged(config::File::with_name("todo_config"))
        .unwrap_or_default()
        .with_merged(config::Environment::with_prefix("NOTION"))?;

    let config: TodoConfig = config.try_into().context("Failed to read config")?;

    let notion_api = NotionApi::new(
        std::env::var("NOTION_API_TOKEN")
            .or(config
                .api_token
                .ok_or(anyhow::anyhow!("No api token from config")))
            .context(
                "No Notion API token found in either the environment variable \
                        `NOTION_API_TOKEN` or the config file!",
            )?,
    )?;

    match opts.command {
        SubCommand::Config => commands::configure::configure(notion_api).await,
        SubCommand::List => list_tasks(notion_api),
        SubCommand::Add => add_task(notion_api),
        SubCommand::Check => complete_task(notion_api),
    }
}

fn list_tasks(_notion_api: NotionApi) -> Result<()> {
    Ok(())
}

fn add_task(_notion_api: NotionApi) -> Result<()> {
    Ok(())
}

fn complete_task(_notion_api: NotionApi) -> Result<()> {
    Ok(())
}
