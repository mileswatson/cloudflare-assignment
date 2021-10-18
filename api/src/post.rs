use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum ContentType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "image")]
    Image,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    title: String,
    username: String,
    content: String,
    #[serde(rename = "type")]
    content_type: ContentType,
}

impl Post {
    pub fn validate(&self) -> core::result::Result<(), &'static str> {
        if self.username.is_empty() {
            Err("Username is empty!")
        } else if self.title.is_empty() {
            Err("Title is empty!")
        } else if self.content.is_empty() {
            Err("Content is empty!")
        } else {
            Ok(())
        }
    }
}
