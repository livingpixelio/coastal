use tauri_plugin_sql::{Migration, MigrationKind};
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migration = Migration {
        version: 1,
        description: "create_initial_tables",
        sql: "CREATE TABLE feeds (id INTEGER PRIMARY KEY, slug TEXT, value TEXT);",
        kind: MigrationKind::Up,
    };

    let migrations = vec![migration];

    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:localdata.db", migrations)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![commands::create_share])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
