use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Store {
    pub name: String,
    pub link: String,
    pub keywords: Vec<String>,
}

impl Clone for Store {
    fn clone(&self) -> Self {
        Store {
            name: self.name.clone(),
            link: self.link.clone(),
            keywords: self.keywords.clone(),
        }
    }
}
