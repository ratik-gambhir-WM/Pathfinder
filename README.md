# Pathfinder

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Environment

Create a local `.env` file from `.env.example` and set `OPENAI_API_KEY` before using the OpenAI-backed parsing features.

`PATHFINDER_DATABASE_PATH` is optional. When it is unset, the Rust app creates `pathfinder.sqlite3` in the platform app data directory.

## SQLite

The Tauri backend initializes SQLite during app startup and stores the connection in managed `AppState`.

Use it from Rust with:

```rust
state.with_db(|db| {
    db.execute("INSERT INTO app_metadata (key, value) VALUES (?1, ?2)", ["example", "ok"])
})?;
```

The `database_status` Tauri command returns the database path and current `PRAGMA user_version`.
