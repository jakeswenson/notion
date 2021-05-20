use crate::TodoConfig;
use anyhow::Result;
use notion::models::search::{FilterProperty, FilterValue, NotionSearch};
use notion::models::{Database, Object};
use notion::{AsIdentifier, NotionApi};
use skim::{Skim, SkimItem, SkimItemReceiver, SkimItemSender, SkimOptions};
use std::borrow::Cow;
use std::ops::Deref;
use std::sync::Arc;

pub async fn configure(notion_api: NotionApi) -> Result<()> {
    let databases: Vec<Database> = notion_api
        .search(NotionSearch::Filter {
            property: FilterProperty::Object,
            value: FilterValue::Database,
        })
        .await?
        .results
        .into_iter()
        .filter_map(|object| match object {
            Object::Database { database } => Some(database),
            _ => None,
        })
        .collect();

    let options = SkimOptions::default();

    let (sender, receiver): (SkimItemSender, SkimItemReceiver) = crossbeam_channel::bounded(500);

    struct SkimDB {
        db: Database,
    }

    impl SkimItem for SkimDB {
        fn text(&self) -> Cow<str> {
            Cow::Owned(self.db.title_plain_text())
        }
    }

    for db in databases {
        sender.send(Arc::new(SkimDB { db }))?;
    }

    // `run_with` would read and show items from the stream
    let selected_items = Skim::run_with(&options, Some(receiver))
        .filter(|out| !out.is_abort)
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    let db = selected_items
        .first()
        .expect("No database selected, aborting...")
        .clone();
    let db: &SkimDB = db
        .deref()
        .as_any()
        .downcast_ref()
        .expect("Couldn't cast back to SkimDB");

    let database_id = db.db.id();

    println!("Selected database's id: {}", database_id);

    let bytes = toml::to_vec(&TodoConfig {
        api_token: None,
        task_database_id: Some(database_id),
    })?;

    std::fs::write("../todo_config.toml", bytes)?;

    Ok(())
}
