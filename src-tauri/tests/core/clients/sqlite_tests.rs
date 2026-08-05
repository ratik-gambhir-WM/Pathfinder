use super::*;

#[test]
fn new_schema_rejects_case_variant_email_duplicates() {
    let connection = Connection::open_in_memory().unwrap();
    run_migrations(&connection).unwrap();

    connection
        .execute(
            "INSERT INTO users (first_name, last_name, email, api_key, role) VALUES (?1, ?2, ?3, ?4, ?5)",
            ["Sam", "Example", "SAM@gmail.com", "test-key", "user"],
        )
        .unwrap();

    let duplicate = connection.execute(
        "INSERT INTO users (first_name, last_name, email, api_key, role) VALUES (?1, ?2, ?3, ?4, ?5)",
        ["Sam", "Example", "sam@gmail.com", "other-key", "user"],
    );

    assert!(duplicate.is_err());
}

#[test]
fn migrations_add_case_insensitive_email_index_to_legacy_schema() {
    let connection = Connection::open_in_memory().unwrap();
    connection
        .execute_batch(
            r#"
            CREATE TABLE users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                first_name TEXT NOT NULL,
                last_name TEXT NOT NULL,
                email TEXT NOT NULL UNIQUE,
                api_key TEXT NOT NULL,
                role TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            "#,
        )
        .unwrap();

    run_migrations(&connection).unwrap();

    let query_plan = connection
        .prepare("EXPLAIN QUERY PLAN SELECT id FROM users WHERE email = ?1 COLLATE NOCASE")
        .unwrap()
        .query_map(["sam@gmail.com"], |row| row.get::<_, String>(3))
        .unwrap()
        .collect::<rusqlite::Result<Vec<_>>>()
        .unwrap();
    let user_version = connection
        .query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0))
        .unwrap();

    assert!(
        query_plan
            .iter()
            .any(|detail| detail.contains("idx_users_email_nocase")),
        "query plan did not use the NOCASE email index: {query_plan:?}"
    );
    assert_eq!(user_version, 4);
}
