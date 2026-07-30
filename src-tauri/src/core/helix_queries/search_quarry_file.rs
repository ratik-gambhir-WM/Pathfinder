use helix_db::dsl::prelude::*;

use super::insert_quarry_file::CHUNK_LABEL;

#[derive(Debug, Clone, PartialEq)]
pub struct ChunkVectorSearch {
    pub user_id: String,
    pub query_embedding: Vec<f32>,
    pub limit: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkKeywordSearch {
    pub user_id: String,
    pub query_text: String,
    pub limit: usize,
}

/// Searches the current user's chunks by embedding distance.
///
/// Results are ordered by ascending `distance`, so the first result is the
/// closest vector match.
pub fn search_chunks_by_vector(search: ChunkVectorSearch) -> Result<DynamicQueryRequest, String> {
    let ChunkVectorSearch {
        user_id,
        query_embedding,
        limit,
    } = search;

    validate_user_id(&user_id)?;
    if query_embedding.is_empty() {
        return Err("query embedding cannot be empty".to_string());
    }

    Ok(search_chunks_by_vector_route(
        user_id,
        query_embedding,
        search_limit_to_i64(limit)?,
    ))
}

#[register]
fn search_chunks_by_vector_route(
    user_id: String,
    query_embedding: Vec<f32>,
    limit: i64,
) -> ReadBatch {
    let _ = (&user_id, &query_embedding, &limit);

    read_batch()
        .var_as(
            "chunks",
            g().vector_search_nodes_with(
                CHUNK_LABEL,
                "embedding",
                PropertyInput::param("query_embedding"),
                Expr::param("limit"),
                Some(PropertyInput::param("user_id")),
            )
            .project(chunk_search_projection("$distance", "distance")),
        )
        .returning(["chunks"])
}

/// Searches the current user's chunks using Helix's BM25 text index.
///
/// Results are ordered by descending `score`, so the first result is the
/// strongest keyword match.
pub fn search_chunks_by_keyword(search: ChunkKeywordSearch) -> Result<DynamicQueryRequest, String> {
    let ChunkKeywordSearch {
        user_id,
        query_text,
        limit,
    } = search;

    validate_user_id(&user_id)?;
    if query_text.trim().is_empty() {
        return Err("keyword query cannot be empty".to_string());
    }

    Ok(search_chunks_by_keyword_route(
        user_id,
        query_text,
        search_limit_to_i64(limit)?,
    ))
}

#[register]
fn search_chunks_by_keyword_route(user_id: String, query_text: String, limit: i64) -> ReadBatch {
    let _ = (&user_id, &query_text, &limit);

    read_batch()
        .var_as(
            "chunks",
            g().text_search_nodes_with(
                CHUNK_LABEL,
                "text",
                PropertyInput::param("query_text"),
                Expr::param("limit"),
                Some(PropertyInput::param("user_id")),
            )
            .project(chunk_search_projection("$score", "score")),
        )
        .returning(["chunks"])
}

fn chunk_search_projection(
    ranking_property: &'static str,
    ranking_alias: &'static str,
) -> Vec<PropertyProjection> {
    vec![
        PropertyProjection::renamed("$id", "id"),
        PropertyProjection::renamed(ranking_property, ranking_alias),
        PropertyProjection::new("chunk_id"),
        PropertyProjection::new("document_id"),
        PropertyProjection::new("user_id"),
        PropertyProjection::new("text"),
        PropertyProjection::new("sequence_number"),
        PropertyProjection::new("page_numbers"),
        PropertyProjection::new("start_offset"),
        PropertyProjection::new("end_offset"),
        PropertyProjection::new("token_count"),
        PropertyProjection::new("content_hash"),
        PropertyProjection::new("section_title"),
    ]
}

fn validate_user_id(user_id: &str) -> Result<(), String> {
    if user_id.trim().is_empty() {
        Err("user_id cannot be empty".to_string())
    } else {
        Ok(())
    }
}

fn search_limit_to_i64(limit: usize) -> Result<i64, String> {
    if limit == 0 {
        return Err("search limit must be greater than zero".to_string());
    }

    i64::try_from(limit).map_err(|_| format!("search limit `{limit}` does not fit in i64"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_search_uses_runtime_embedding_limit_and_user_partition() {
        let request = search_chunks_by_vector(ChunkVectorSearch {
            user_id: "user-1".to_string(),
            query_embedding: vec![0.1, 0.2, 0.3],
            limit: 8,
        })
        .unwrap();
        let parameters = request.parameters.as_ref().unwrap();
        let json = serde_json::to_string(&request).unwrap();

        assert_eq!(
            parameters.get("user_id"),
            Some(&DynamicQueryValue::String("user-1".to_string()))
        );
        assert_eq!(parameters.get("limit"), Some(&DynamicQueryValue::I64(8)));
        assert!(parameters.contains_key("query_embedding"));
        assert!(json.contains("VectorSearchNodes"));
        assert!(json.contains("$distance"));
        assert!(json.contains("user_id"));
    }

    #[test]
    fn keyword_search_uses_runtime_text_limit_and_user_partition() {
        let request = search_chunks_by_keyword(ChunkKeywordSearch {
            user_id: "user-1".to_string(),
            query_text: "material adverse change".to_string(),
            limit: 5,
        })
        .unwrap();
        let parameters = request.parameters.as_ref().unwrap();
        let json = serde_json::to_string(&request).unwrap();

        assert_eq!(
            parameters.get("query_text"),
            Some(&DynamicQueryValue::String(
                "material adverse change".to_string()
            ))
        );
        assert_eq!(parameters.get("limit"), Some(&DynamicQueryValue::I64(5)));
        assert!(json.contains("TextSearchNodes"));
        assert!(json.contains("$score"));
        assert!(json.contains("user_id"));
    }

    #[test]
    fn search_inputs_are_validated() {
        assert!(search_chunks_by_vector(ChunkVectorSearch {
            user_id: "user-1".to_string(),
            query_embedding: Vec::new(),
            limit: 5,
        })
        .is_err());
        assert!(search_chunks_by_keyword(ChunkKeywordSearch {
            user_id: "user-1".to_string(),
            query_text: "   ".to_string(),
            limit: 5,
        })
        .is_err());
        assert!(search_chunks_by_keyword(ChunkKeywordSearch {
            user_id: "user-1".to_string(),
            query_text: "term".to_string(),
            limit: 0,
        })
        .is_err());
    }

    #[test]
    fn registered_bundle_contains_chunk_search_routes() {
        let bundle = helix_db::query_generator::build_query_bundle().unwrap();

        for route in [
            "search_chunks_by_vector_route",
            "search_chunks_by_keyword_route",
        ] {
            assert!(bundle.read_routes.contains_key(route));
        }
    }
}
