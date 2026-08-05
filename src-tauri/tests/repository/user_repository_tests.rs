use super::*;

#[test]
fn query_user_by_email_matches_email_case_insensitively() {
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

            INSERT INTO users (first_name, last_name, email, api_key, role)
            VALUES ('Sam', 'Example', 'SAM@gmail.com', 'test-key', 'user');
            "#,
        )
        .unwrap();

    let user = query_user_by_email(&connection, "sam@gmail.com")
        .unwrap()
        .expect("case-insensitive email lookup should find the existing user");

    assert_eq!(user.email, "SAM@gmail.com");
}
