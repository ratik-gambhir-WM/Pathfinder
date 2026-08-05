use helix_db::dsl::prelude::*;

use crate::core::{helix_queries::user::add_user::USER_LABEL, nodes::deal_node::DealNode};

pub const DEAL_LABEL: &str = "Deal";
pub const USER_HAS_DEAL_LABEL: &str = "HAS_DEAL";

/// Builds an ID-keyed upsert for a Helix `Deal` node.
///
/// All properties mirror the SQLite `deals` row. An existing node is updated;
/// when no matching ID exists, a new node is created. The resolved Helix user
/// is connected to the deal with one idempotent `HAS_DEAL` edge.
pub fn add_deal(deal: DealNode, user_id: i64) -> Result<DynamicQueryRequest, String> {
    let DealNode {
        id,
        deal_name,
        main_data_room_folder,
        deal_type,
        pe_firm,
        status,
        target_company,
        buyer_or_platform_company,
        parent_or_seller_company,
        carve_out_business,
        created_at,
        updated_at,
    } = deal;

    validate_deal_id(id)?;
    validate_user_id(user_id)?;

    Ok(add_deal_mutation(
        id,
        user_id,
        deal_name,
        main_data_room_folder,
        deal_type,
        pe_firm,
        status,
        optional_string_property(target_company),
        optional_string_property(buyer_or_platform_company),
        optional_string_property(parent_or_seller_company),
        optional_string_property(carve_out_business),
        created_at,
        updated_at,
    ))
}

#[register]
fn add_deal_mutation(
    id: i64,
    user_id: i64,
    deal_name: String,
    main_data_room_folder: String,
    deal_type: String,
    pe_firm: String,
    status: String,
    target_company: PropertyValue,
    buyer_or_platform_company: PropertyValue,
    parent_or_seller_company: PropertyValue,
    carve_out_business: PropertyValue,
    created_at: String,
    updated_at: String,
) -> WriteBatch {
    let _ = (
        &id,
        &user_id,
        &deal_name,
        &main_data_room_folder,
        &deal_type,
        &pe_firm,
        &status,
        &target_company,
        &buyer_or_platform_company,
        &parent_or_seller_company,
        &carve_out_business,
        &created_at,
        &updated_at,
    );

    write_batch()
        .var_as(
            "existing_deal",
            g().n_with_label(DEAL_LABEL)
                .where_(Predicate::eq_param("id", "id")),
        )
        .var_as_if(
            "updated_deal",
            BatchCondition::VarNotEmpty("existing_deal".to_string()),
            g().n(NodeRef::var("existing_deal"))
                .set_property("id", PropertyInput::param("id"))
                .set_property("deal_name", PropertyInput::param("deal_name"))
                .set_property(
                    "main_data_room_folder",
                    PropertyInput::param("main_data_room_folder"),
                )
                .set_property("deal_type", PropertyInput::param("deal_type"))
                .set_property("pe_firm", PropertyInput::param("pe_firm"))
                .set_property("status", PropertyInput::param("status"))
                .set_property("target_company", PropertyInput::param("target_company"))
                .set_property(
                    "buyer_or_platform_company",
                    PropertyInput::param("buyer_or_platform_company"),
                )
                .set_property(
                    "parent_or_seller_company",
                    PropertyInput::param("parent_or_seller_company"),
                )
                .set_property(
                    "carve_out_business",
                    PropertyInput::param("carve_out_business"),
                )
                .set_property("created_at", PropertyInput::param("created_at"))
                .set_property("updated_at", PropertyInput::param("updated_at"))
                .project(deal_projection()),
        )
        .var_as_if(
            "created_deal",
            BatchCondition::VarEmpty("existing_deal".to_string()),
            g().add_n(
                DEAL_LABEL,
                vec![
                    ("id", PropertyInput::param("id")),
                    ("deal_name", PropertyInput::param("deal_name")),
                    (
                        "main_data_room_folder",
                        PropertyInput::param("main_data_room_folder"),
                    ),
                    ("deal_type", PropertyInput::param("deal_type")),
                    ("pe_firm", PropertyInput::param("pe_firm")),
                    ("status", PropertyInput::param("status")),
                    ("target_company", PropertyInput::param("target_company")),
                    (
                        "buyer_or_platform_company",
                        PropertyInput::param("buyer_or_platform_company"),
                    ),
                    (
                        "parent_or_seller_company",
                        PropertyInput::param("parent_or_seller_company"),
                    ),
                    (
                        "carve_out_business",
                        PropertyInput::param("carve_out_business"),
                    ),
                    ("created_at", PropertyInput::param("created_at")),
                    ("updated_at", PropertyInput::param("updated_at")),
                ],
            )
            .project(deal_projection()),
        )
        .var_as(
            "deal",
            g().n_with_label(DEAL_LABEL)
                .where_(Predicate::eq_param("id", "id")),
        )
        .var_as(
            "user",
            g().n_with_label(USER_LABEL)
                .where_(Predicate::eq_param("id", "user_id")),
        )
        .var_as(
            "existing_user_deal",
            g().e_with_label(USER_HAS_DEAL_LABEL)
                .where_(Predicate::eq_param("user_id", "user_id"))
                .where_(Predicate::eq_param("deal_id", "id")),
        )
        .var_as_if(
            "user_has_deal",
            BatchCondition::VarEmpty("existing_user_deal".to_string()),
            g().n(NodeRef::var("user")).add_e(
                USER_HAS_DEAL_LABEL,
                NodeRef::var("deal"),
                vec![
                    ("user_id", PropertyInput::param("user_id")),
                    ("deal_id", PropertyInput::param("id")),
                ],
            ),
        )
        .returning([
            "updated_deal",
            "created_deal",
            "existing_user_deal",
            "user_has_deal",
        ])
}

/// Builds a Helix lookup equivalent to SQLite's `get_deal_by_id` query.
pub fn get_deal_by_id(deal_id: i64) -> Result<DynamicQueryRequest, String> {
    validate_deal_id(deal_id)?;
    Ok(get_deal_by_id_query(deal_id))
}

#[register]
fn get_deal_by_id_query(id: i64) -> ReadBatch {
    let _ = &id;

    read_batch()
        .var_as(
            "deal",
            g().n_with_label(DEAL_LABEL)
                .where_(Predicate::eq_param("id", "id"))
                .limit(1)
                .project(deal_projection()),
        )
        .returning(["deal"])
}

/// Creates the deal node indexes and user-to-deal relationship indexes.
#[register]
pub fn create_deal_indexes() -> WriteBatch {
    write_batch()
        .var_as(
            "deal_id_unique",
            g().create_index_if_not_exists(IndexSpec::node_unique_equality(DEAL_LABEL, "id")),
        )
        .var_as(
            "deal_type",
            g().create_index_if_not_exists(IndexSpec::node_equality(DEAL_LABEL, "deal_type")),
        )
        .var_as(
            "deal_pe_firm",
            g().create_index_if_not_exists(IndexSpec::node_equality(DEAL_LABEL, "pe_firm")),
        )
        .var_as(
            "deal_updated_at",
            g().create_index_if_not_exists(IndexSpec::node_equality(DEAL_LABEL, "updated_at")),
        )
        .var_as(
            "user_has_deal_user_id",
            g().create_index_if_not_exists(IndexSpec::edge_equality(
                USER_HAS_DEAL_LABEL,
                "user_id",
            )),
        )
        .var_as(
            "user_has_deal_deal_id",
            g().create_index_if_not_exists(IndexSpec::edge_equality(
                USER_HAS_DEAL_LABEL,
                "deal_id",
            )),
        )
}

fn deal_projection() -> Vec<PropertyProjection> {
    vec![
        PropertyProjection::renamed("$id", "helix_id"),
        PropertyProjection::new("id"),
        PropertyProjection::new("deal_name"),
        PropertyProjection::new("main_data_room_folder"),
        PropertyProjection::new("deal_type"),
        PropertyProjection::new("pe_firm"),
        PropertyProjection::new("status"),
        PropertyProjection::new("target_company"),
        PropertyProjection::new("buyer_or_platform_company"),
        PropertyProjection::new("parent_or_seller_company"),
        PropertyProjection::new("carve_out_business"),
        PropertyProjection::new("created_at"),
        PropertyProjection::new("updated_at"),
    ]
}

fn optional_string_property(value: Option<String>) -> PropertyValue {
    value.map_or(PropertyValue::Null, PropertyValue::String)
}

fn validate_deal_id(id: i64) -> Result<(), String> {
    if id <= 0 {
        Err("deal id must be greater than zero".to_string())
    } else {
        Ok(())
    }
}

fn validate_user_id(id: i64) -> Result<(), String> {
    if id <= 0 {
        Err("user id must be greater than zero".to_string())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
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
}
