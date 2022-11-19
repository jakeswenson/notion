# notion
[![Build](https://github.com/jakeswenson/notion/actions/workflows/build.yml/badge.svg)](https://github.com/jakeswenson/notion/actions/workflows/build.yml)
[![Crates.io](https://img.shields.io/crates/v/notion?style=for-the-badge)](https://crates.io/crates/notion)

Notion API client library for rust.

This project is under active development and this README will be updated as this library gets closer to a reliable state.
However, if you're really eager see the example todo cli application provided in [examples/todo](examples/todo).

## Docs

The generated documentation site is available here: https://docs.rs/notion/

## Building

```bash
cargo build
```

### Pull Request builds



## Testing

We are in the process of moving to [wiremock](https://docs.rs/wiremock/latest/wiremock/) based notion api testing.
Existing tests use a private notion org, and expect an environment variable set of `NOTION_API_TOKEN`.

We understand that right now this is a bit painful, but any help in this migration journey is very welcome!

## Contributing

Contributions are always welcome!
If you have an idea, it's best to float it by us before working on it to ensure no effort is wasted.
If there's already an open issue for it, knock yourself out.

If you have any questions, feel free to use [Discussions](https://github.com/jakeswenson/notion/discussions).
Please don't hesitate to ask questions!
