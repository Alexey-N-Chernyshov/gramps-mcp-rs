use super::{GrampsDate, Handle};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonName {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub name_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    pub surname_list: Vec<Surname>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<GrampsDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Surname {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origintype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector: Option<String>,
}

/// Request body for creating a new person.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreatePersonRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_name: Option<PersonName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gramps_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_list: Option<Vec<Handle>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citation_list: Option<Vec<Handle>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ref_list: Option<Vec<serde_json::Value>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_person_request_serializes_name() {
        let req = CreatePersonRequest {
            primary_name: Some(PersonName {
                first_name: Some("Anna".into()),
                surname_list: vec![Surname {
                    surname: Some("Karenina".into()),
                    ..Default::default()
                }],
                name_type: Some("Birth Name".into()),
                ..Default::default()
            }),
            gender: Some(2),
            ..Default::default()
        };
        let v = serde_json::to_value(&req).unwrap();
        assert_eq!(v["gender"], 2);
        assert_eq!(v["primary_name"]["first_name"], "Anna");
        assert_eq!(v["primary_name"]["surname_list"][0]["surname"], "Karenina");
        assert_eq!(v["primary_name"]["type"], "Birth Name");
    }
}
