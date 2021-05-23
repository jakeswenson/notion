# Notion database todo example

This example is builds a todo list using a notion database.

## Setup your notion api token

Create an `internal` integration here: https://www.notion.so/my-integrations

```bash
  export NOTION_API_TOKEN='secret_token_here'
```
> Notice the space before the export command. 
> This will prevent your terminal from storing this token in your shell history...

## Selecting the database to use

```bash
cargo run --example todo -- config
```
