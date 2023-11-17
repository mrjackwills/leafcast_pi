use serde::{Deserialize, Serialize};

// Valid>>String is the unique hex string that is created by the staticpi server
#[derive(Debug)]
pub enum MessageValues {
    Valid(ParsedMessage, String),
    Invalid(ErrorData),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "name", content = "body")]
pub enum ParsedMessage {
    Photo,
    ForceUpdate,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct StructuredMessage {
    data: Option<ParsedMessage>,
    error: Option<ErrorData>,
    unique: String,
}

// TODO
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "error", content = "message")]
pub enum ErrorData {
    Something(String),
}

pub fn to_struct(input: &str) -> Option<MessageValues> {
    let user_serialized = serde_json::from_str::<StructuredMessage>(input);
    if let Ok(data) = user_serialized {
        if let Some(message) = data.error {
            return Some(MessageValues::Invalid(message));
        }
        if let Some(message) = data.data {
            return Some(MessageValues::Valid(message, data.unique));
        }
        None
    } else {
        let error_serialized = serde_json::from_str::<ErrorData>(input);
        error_serialized.map_or(None, |data| Some(MessageValues::Invalid(data)))
    }
}

/// message_incoming
///
/// cargo watch -q -c -w src/ -x 'test message_incoming -- --test-threads=1 --nocapture'
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn message_incoming_parse_invalid() {
        let data = r"";
        let result = to_struct(data);
        assert!(result.is_none());

        let data = r"{}";
        let result = to_struct(data);
        assert!(result.is_none());
    }

    #[test]
    fn message_incoming_parse_photo_ok() {
        let data = r#"
            {
            	"data": {
            		"name" : "photo"
				},
				"unique": "unique_hex_string"
            }"#;
        let result = to_struct(data);
        assert!(result.is_some());
        let result = result.unwrap();
        match result {
            MessageValues::Valid(ParsedMessage::Photo, unique) => {
                assert_eq!(unique, "unique_hex_string");
            }
            _ => unreachable!("Shouldn't have matched this"),
        };
    }

    #[test]
    fn message_incoming_parse_photo_body_err() {
        let data = r#"
            {
            	"data": {
            		"name" : "photo",
					"body": {
						"anything":"here"
					}
				},
				"unique": "unique_hex_string"
            }"#;
        let result = to_struct(data);
        assert!(result.is_none());
    }

    #[test]
    fn message_incoming_parse_photo_unique_err() {
        let data = r#"
            {
            	"data": {
            		"name" : "photo",
				}
				
            }"#;
        let result = to_struct(data);
        assert!(result.is_none());

        let data = r#"
		{
			"data": {
				"name" : "photo",
			},
			"unique": ""
			
		}"#;
        let result = to_struct(data);
        assert!(result.is_none());

        let data = r#"
	{
		"data": {
			"name" : "photo",
		},
		"unique": true
		
	}"#;
        let result = to_struct(data);
        assert!(result.is_none());
    }

    #[test]
    fn message_incoming_parse_force_update_ok() {
        let data = r#"
            {
            	"data": {
            		"name" : "force_update"
				},
				"unique": "unique_hex_string"
            }"#;
        let result = to_struct(data);
        assert!(result.is_some());
        let result = result.unwrap();
        match result {
            MessageValues::Valid(ParsedMessage::ForceUpdate, unique) => {
                assert_eq!(unique, "unique_hex_string");
            }
            _ => unreachable!("Shouldn't have matched this"),
        };
    }

    #[test]
    fn message_incoming_parse_force_update_unique_err() {
        let data = r#"
		{
			"data": {
				"name" : "force_update",
			}
		}"#;
        let result = to_struct(data);
        assert!(result.is_none());

        let data = r#"
	{
		"data": {
			"name" : "force_update",
		},
		"unique": ""
	}"#;
        let result = to_struct(data);
        assert!(result.is_none());

        let data = r#"
{
	"data": {
		"name" : "force_update",
	},
	"unique": true
	
}"#;
        let result = to_struct(data);
        assert!(result.is_none());
    }
}
