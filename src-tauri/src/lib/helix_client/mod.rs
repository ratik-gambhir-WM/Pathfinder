use helix_rs::{HelixDB, HelixDBClient};
use serde::Serialize;
use serde_json::Value;


#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct InsertUserInput {
    pub name: String,
    pub email: String,
    #[serde(rename = "apiKey")]
    pub api_key: String,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct GetUserByEmailInput {
    pub email: String,
}


pub struct HelixClient {
    pub helix_db: HelixDB


}

impl HelixClient {
    pub fn new() -> Self {
        let helix_db: HelixDB = HelixDB::new(Some("http://localhost"), Some(6969), None);
        Self { helix_db }
    }

    pub async fn add_user(
        &self,
        name: Option<&String>,
        email: Option<&String>,
        api_key: Option<&String>,
    ) -> Result<(), String> {
        let input = build_insert_user_input(name, email, api_key)?;

        let result: Value = self.helix_db
            .query("InsertUser", &input)
            .await
            .map_err(|err| err.to_string())?;

        let pretty = serde_json::to_string_pretty(&result).map_err(|err| err.to_string())?;
        println!("{pretty}");

        Ok(())
    }

    pub async fn get_user(&self, email: Option<&String>) -> Result<(), String> {
        let input = build_get_user_by_email_input(email)?;

        let result: Value = self.helix_db
            .query("GetUserByEmail", &input)
            .await
            .map_err(|err| err.to_string())?;

        let pretty = serde_json::to_string_pretty(&result).map_err(|err| err.to_string())?;
        println!("{pretty}");

        Ok(())
    }
}

fn build_insert_user_input(
    name: Option<&String>,
    email: Option<&String>,
    api_key: Option<&String>,
) -> Result<InsertUserInput, String> {
    Ok(InsertUserInput {
        name: name
            .cloned()
            .ok_or_else(|| "missing required argument: <name>".to_string())?,
        email: email
            .cloned()
            .ok_or_else(|| "missing required argument: <email>".to_string())?,
        api_key: api_key
            .cloned()
            .ok_or_else(|| "missing required argument: <api_key>".to_string())?,
    })
}

fn build_get_user_by_email_input(email: Option<&String>) -> Result<GetUserByEmailInput, String> {
    Ok(GetUserByEmailInput {
        email: email
            .cloned()
            .ok_or_else(|| "missing required argument: <email>".to_string())?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn build_insert_user_input_returns_struct_when_all_fields_are_present() {
        let name = "Ada".to_string();
        let email = "ada@example.com".to_string();
        let api_key = "sk-test".to_string();

        let input = build_insert_user_input(Some(&name), Some(&email), Some(&api_key)).unwrap();

        assert_eq!(
            input,
            InsertUserInput {
                name,
                email,
                api_key,
            }
        );
    }

    #[test]
    fn build_insert_user_input_returns_error_when_name_is_missing() {
        let email = "ada@example.com".to_string();
        let api_key = "sk-test".to_string();

        let result = build_insert_user_input(None, Some(&email), Some(&api_key));

        assert_eq!(result.unwrap_err(), "missing required argument: <name>");
    }

    #[test]
    fn insert_user_input_serializes_api_key_with_expected_field_name() {
        let input = InsertUserInput {
            name: "Ada".to_string(),
            email: "ada@example.com".to_string(),
            api_key: "sk-test".to_string(),
        };

        let value = serde_json::to_value(&input).unwrap();

        assert_eq!(
            value,
            json!({
                "name": "Ada",
                "email": "ada@example.com",
                "apiKey": "sk-test",
            })
        );
    }

    #[test]
    fn build_get_user_by_email_input_returns_struct_when_email_is_present() {
        let email = "ada@example.com".to_string();

        let input = build_get_user_by_email_input(Some(&email)).unwrap();

        assert_eq!(
            input,
            GetUserByEmailInput {
                email,
            }
        );
    }

    #[test]
    fn build_get_user_by_email_input_returns_error_when_email_is_missing() {
        let result = build_get_user_by_email_input(None);

        assert_eq!(result.unwrap_err(), "missing required argument: <email>");
    }
}
