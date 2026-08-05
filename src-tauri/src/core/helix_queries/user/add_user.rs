use helix_db::dsl::prelude::*;

use crate::core::nodes::user_node::UserNode;

pub const USER_LABEL: &str = "User";

/// Builds an email-keyed upsert for a Helix `User` node.
///
/// All properties mirror the SQLite `users` row. An existing node is updated;
/// when no matching email exists, a new node is created.
pub fn add_user(user: UserNode) -> Result<DynamicQueryRequest, String> {
    let UserNode {
        id,
        first_name,
        last_name,
        email,
        api_key,
        role,
        created_at,
        updated_at,
    } = user;

    if email.trim().is_empty() {
        return Err("user email cannot be empty".to_string());
    }

    Ok(add_user_mutation(
        id, first_name, last_name, email, api_key, role, created_at, updated_at,
    ))
}

/// Builds a Helix lookup for one `User` node by its exact email address.
pub fn get_user_by_email(email: String) -> Result<DynamicQueryRequest, String> {
    if email.trim().is_empty() {
        return Err("user email cannot be empty".to_string());
    }

    Ok(get_user_by_email_query(email))
}

#[register]
fn get_user_by_email_query(email: String) -> ReadBatch {
    let _ = &email;

    read_batch()
        .var_as(
            "user",
            g().n_with_label(USER_LABEL)
                .where_(Predicate::eq_param("email", "email"))
                .limit(1)
                .project(user_projection()),
        )
        .returning(["user"])
}

#[register]
fn add_user_mutation(
    id: i64,
    first_name: String,
    last_name: String,
    email: String,
    api_key: String,
    role: String,
    created_at: String,
    updated_at: String,
) -> WriteBatch {
    let _ = (
        &id,
        &first_name,
        &last_name,
        &email,
        &api_key,
        &role,
        &created_at,
        &updated_at,
    );

    write_batch()
        .var_as(
            "existing_user",
            g().n_with_label(USER_LABEL)
                .where_(Predicate::eq_param("email", "email")),
        )
        .var_as_if(
            "updated_user",
            BatchCondition::VarNotEmpty("existing_user".to_string()),
            g().n(NodeRef::var("existing_user"))
                .set_property("id", PropertyInput::param("id"))
                .set_property("first_name", PropertyInput::param("first_name"))
                .set_property("last_name", PropertyInput::param("last_name"))
                .set_property("email", PropertyInput::param("email"))
                .set_property("api_key", PropertyInput::param("api_key"))
                .set_property("role", PropertyInput::param("role"))
                .set_property("created_at", PropertyInput::param("created_at"))
                .set_property("updated_at", PropertyInput::param("updated_at"))
                .project(user_projection()),
        )
        .var_as_if(
            "created_user",
            BatchCondition::VarEmpty("existing_user".to_string()),
            g().add_n(
                USER_LABEL,
                vec![
                    ("id", PropertyInput::param("id")),
                    ("first_name", PropertyInput::param("first_name")),
                    ("last_name", PropertyInput::param("last_name")),
                    ("email", PropertyInput::param("email")),
                    ("api_key", PropertyInput::param("api_key")),
                    ("role", PropertyInput::param("role")),
                    ("created_at", PropertyInput::param("created_at")),
                    ("updated_at", PropertyInput::param("updated_at")),
                ],
            )
            .project(user_projection()),
        )
        .returning(["updated_user", "created_user"])
}

/// Creates the unique indexes that mirror the SQLite user constraints.
#[register]
pub fn create_user_indexes() -> WriteBatch {
    write_batch()
        .var_as(
            "user_id_unique",
            g().create_index_if_not_exists(IndexSpec::node_unique_equality(USER_LABEL, "id")),
        )
        .var_as(
            "user_email_unique",
            g().create_index_if_not_exists(IndexSpec::node_unique_equality(USER_LABEL, "email")),
        )
}

fn user_projection() -> Vec<PropertyProjection> {
    vec![
        PropertyProjection::renamed("$id", "helix_id"),
        PropertyProjection::new("id"),
        PropertyProjection::new("first_name"),
        PropertyProjection::new("last_name"),
        PropertyProjection::new("email"),
        PropertyProjection::new("api_key"),
        PropertyProjection::new("role"),
        PropertyProjection::new("created_at"),
        PropertyProjection::new("updated_at"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user() -> UserNode {
        UserNode {
            id: 42,
            first_name: "Sam".to_string(),
            last_name: "Example".to_string(),
            email: "sam@example.com".to_string(),
            api_key: "test-api-key".to_string(),
            role: "analyst".to_string(),
            created_at: "2026-08-03 15:00:00".to_string(),
            updated_at: "2026-08-03 15:30:00".to_string(),
        }
    }

    #[test]
    fn user_query_accepts_and_decomposes_user_node() {
        let request = add_user(user()).unwrap();
        let parameters = request.parameters.unwrap();

        assert_eq!(parameters.get("id"), Some(&DynamicQueryValue::I64(42)));
        assert_eq!(
            parameters.get("first_name"),
            Some(&DynamicQueryValue::String("Sam".to_string()))
        );
        assert_eq!(
            parameters.get("last_name"),
            Some(&DynamicQueryValue::String("Example".to_string()))
        );
        assert_eq!(
            parameters.get("email"),
            Some(&DynamicQueryValue::String("sam@example.com".to_string()))
        );
        assert_eq!(
            parameters.get("api_key"),
            Some(&DynamicQueryValue::String("test-api-key".to_string()))
        );
        assert_eq!(
            parameters.get("role"),
            Some(&DynamicQueryValue::String("analyst".to_string()))
        );
        assert_eq!(
            parameters.get("created_at"),
            Some(&DynamicQueryValue::String(
                "2026-08-03 15:00:00".to_string()
            ))
        );
        assert_eq!(
            parameters.get("updated_at"),
            Some(&DynamicQueryValue::String(
                "2026-08-03 15:30:00".to_string()
            ))
        );
    }

    #[test]
    fn user_query_updates_or_creates_by_email() {
        let request = add_user(user()).unwrap();
        let json = serde_json::to_value(request).unwrap();
        let queries = json["query"]["queries"].as_array().unwrap();

        assert_eq!(queries.len(), 3);
        assert_eq!(
            json["query"]["returns"],
            serde_json::json!(["updated_user", "created_user"])
        );
        assert_eq!(queries[0]["Query"]["name"], "existing_user");
        assert_eq!(
            queries[1]["Query"]["condition"],
            serde_json::json!({"VarNotEmpty": "existing_user"})
        );
        assert_eq!(queries[1]["Query"]["name"], "updated_user");
        assert_eq!(
            queries[2]["Query"]["condition"],
            serde_json::json!({"VarEmpty": "existing_user"})
        );
        assert_eq!(queries[2]["Query"]["name"], "created_user");

        let serialized = serde_json::to_string(&json).unwrap();
        for field in [
            "id",
            "first_name",
            "last_name",
            "email",
            "api_key",
            "role",
            "created_at",
            "updated_at",
        ] {
            assert!(serialized.contains(field));
        }
    }

    #[test]
    fn user_query_rejects_an_empty_email() {
        let mut user = user();
        user.email = "   ".to_string();

        assert_eq!(
            add_user(user).unwrap_err(),
            "user email cannot be empty".to_string()
        );
    }

    #[test]
    fn get_user_query_filters_by_email_and_projects_one_user() {
        let request = get_user_by_email("sam@example.com".to_string()).unwrap();
        let parameters = request.parameters.as_ref().unwrap();
        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(
            parameters.get("email"),
            Some(&DynamicQueryValue::String("sam@example.com".to_string()))
        );
        assert_eq!(json["query"]["returns"], serde_json::json!(["user"]));
        assert_eq!(json["query"]["queries"][0]["Query"]["name"], "user");
        assert_eq!(
            json["query"]["queries"][0]["Query"]["steps"][2],
            serde_json::json!({"Limit": 1})
        );
    }

    #[test]
    fn get_user_query_rejects_an_empty_email() {
        assert_eq!(
            get_user_by_email("   ".to_string()).unwrap_err(),
            "user email cannot be empty"
        );
    }

    #[test]
    fn user_index_query_uses_unique_id_and_email_indexes() {
        let json = serde_json::to_value(create_user_indexes()).unwrap();
        let queries = json["query"]["queries"].as_array().unwrap();

        assert_eq!(queries.len(), 2);
        for (query, (name, property)) in queries
            .iter()
            .zip([("user_id_unique", "id"), ("user_email_unique", "email")])
        {
            assert_eq!(query["Query"]["name"], name);
            assert_eq!(
                query["Query"]["steps"],
                serde_json::json!([{
                    "CreateIndex": {
                        "spec": {
                            "NodeEquality": {
                                "label": USER_LABEL,
                                "property": property,
                                "unique": true
                            }
                        },
                        "if_not_exists": true
                    }
                }])
            );
        }
    }

    #[test]
    fn registered_bundle_contains_user_query_routes() {
        let bundle = helix_db::query_generator::build_query_bundle().unwrap();

        for route in ["add_user_mutation", "create_user_indexes"] {
            assert!(bundle.write_routes.contains_key(route));
        }
        assert!(bundle.read_routes.contains_key("get_user_by_email_query"));
    }
}
