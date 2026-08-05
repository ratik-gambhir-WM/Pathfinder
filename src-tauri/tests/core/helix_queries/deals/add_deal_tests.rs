use super::*;

fn deal() -> DealNode {
    DealNode {
        id: 17,
        deal_name: "Project Aurora".to_string(),
        main_data_room_folder: "/data-rooms/project-aurora".to_string(),
        deal_type: "Buy-side".to_string(),
        pe_firm: "Example Capital".to_string(),
        status: "active".to_string(),
        target_company: Some("Aurora Systems".to_string()),
        buyer_or_platform_company: Some("Northstar Platform".to_string()),
        parent_or_seller_company: None,
        carve_out_business: None,
        created_at: "2026-08-03 15:00:00".to_string(),
        updated_at: "2026-08-03 15:30:00".to_string(),
    }
}

#[test]
fn deal_query_accepts_and_decomposes_deal_node() {
    let request = add_deal(deal(), 42).unwrap();
    let parameters = request.parameters.unwrap();

    assert_eq!(parameters.get("id"), Some(&DynamicQueryValue::I64(17)));
    assert_eq!(parameters.get("user_id"), Some(&DynamicQueryValue::I64(42)));
    assert_eq!(
        parameters.get("deal_name"),
        Some(&DynamicQueryValue::String("Project Aurora".to_string()))
    );
    assert_eq!(
        parameters.get("main_data_room_folder"),
        Some(&DynamicQueryValue::String(
            "/data-rooms/project-aurora".to_string()
        ))
    );
    assert_eq!(
        parameters.get("target_company"),
        Some(&DynamicQueryValue::String("Aurora Systems".to_string()))
    );
    assert_eq!(
        parameters.get("parent_or_seller_company"),
        Some(&DynamicQueryValue::Null)
    );
    assert_eq!(
        parameters.get("carve_out_business"),
        Some(&DynamicQueryValue::Null)
    );
}

#[test]
fn deal_query_updates_or_creates_by_id() {
    let request = add_deal(deal(), 42).unwrap();
    let json = serde_json::to_value(request).unwrap();
    let queries = json["query"]["queries"].as_array().unwrap();

    assert_eq!(queries.len(), 7);
    assert_eq!(
        json["query"]["returns"],
        serde_json::json!([
            "updated_deal",
            "created_deal",
            "existing_user_deal",
            "user_has_deal"
        ])
    );
    assert_eq!(queries[0]["Query"]["name"], "existing_deal");
    assert_eq!(
        queries[1]["Query"]["condition"],
        serde_json::json!({"VarNotEmpty": "existing_deal"})
    );
    assert_eq!(queries[1]["Query"]["name"], "updated_deal");
    assert_eq!(
        queries[2]["Query"]["condition"],
        serde_json::json!({"VarEmpty": "existing_deal"})
    );
    assert_eq!(queries[2]["Query"]["name"], "created_deal");
    assert_eq!(queries[3]["Query"]["name"], "deal");
    assert_eq!(queries[4]["Query"]["name"], "user");
    assert_eq!(queries[5]["Query"]["name"], "existing_user_deal");
    assert_eq!(queries[6]["Query"]["name"], "user_has_deal");
    assert_eq!(
        queries[6]["Query"]["condition"],
        serde_json::json!({"VarEmpty": "existing_user_deal"})
    );

    let serialized = serde_json::to_string(&json).unwrap();
    assert!(serialized.contains(USER_LABEL));
    assert!(serialized.contains(USER_HAS_DEAL_LABEL));
    assert!(serialized.contains("user_id"));
    assert!(serialized.contains("deal_id"));
}

#[test]
fn get_deal_query_filters_by_id_and_projects_the_deal() {
    let request = get_deal_by_id(17).unwrap();
    let parameters = request.parameters.as_ref().unwrap();
    let json = serde_json::to_value(&request).unwrap();

    assert_eq!(parameters.get("id"), Some(&DynamicQueryValue::I64(17)));
    assert_eq!(json["query"]["returns"], serde_json::json!(["deal"]));
    assert_eq!(json["query"]["queries"][0]["Query"]["name"], "deal");
    assert!(serde_json::to_string(&json).unwrap().contains("deal_name"));
}

#[test]
fn deal_queries_reject_invalid_ids() {
    let mut invalid_deal = deal();
    invalid_deal.id = 0;

    assert_eq!(
        add_deal(invalid_deal, 42).unwrap_err(),
        "deal id must be greater than zero"
    );
    assert_eq!(
        add_deal(deal(), 0).unwrap_err(),
        "user id must be greater than zero"
    );
    assert_eq!(
        get_deal_by_id(-1).unwrap_err(),
        "deal id must be greater than zero"
    );
}

#[test]
fn deal_index_query_includes_node_and_relationship_indexes() {
    let json = serde_json::to_value(create_deal_indexes()).unwrap();
    let queries = json["query"]["queries"].as_array().unwrap();
    let expected_node_indexes = [
        ("deal_id_unique", "id", true),
        ("deal_type", "deal_type", false),
        ("deal_pe_firm", "pe_firm", false),
        ("deal_updated_at", "updated_at", false),
    ];

    assert_eq!(queries.len(), 6);
    for (query, (name, property, unique)) in queries.iter().take(4).zip(expected_node_indexes) {
        assert_eq!(query["Query"]["name"], name);
        assert_eq!(
            query["Query"]["steps"],
            serde_json::json!([{
                "CreateIndex": {
                    "spec": {
                        "NodeEquality": {
                            "label": DEAL_LABEL,
                            "property": property,
                            "unique": unique
                        }
                    },
                    "if_not_exists": true
                }
            }])
        );
    }

    for (query, (name, property)) in queries.iter().skip(4).zip([
        ("user_has_deal_user_id", "user_id"),
        ("user_has_deal_deal_id", "deal_id"),
    ]) {
        assert_eq!(query["Query"]["name"], name);
        assert_eq!(
            query["Query"]["steps"],
            serde_json::json!([{
                "CreateIndex": {
                    "spec": {
                        "EdgeEquality": {
                            "label": USER_HAS_DEAL_LABEL,
                            "property": property
                        }
                    },
                    "if_not_exists": true
                }
            }])
        );
    }
}

#[test]
fn registered_bundle_contains_deal_query_routes() {
    let bundle = helix_db::query_generator::build_query_bundle().unwrap();

    for route in ["add_deal_mutation", "create_deal_indexes"] {
        assert!(bundle.write_routes.contains_key(route));
    }
    assert!(bundle.read_routes.contains_key("get_deal_by_id_query"));
}
