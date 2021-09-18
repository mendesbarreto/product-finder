use std::error::Error;

use rust_embed::EmbeddedFile;

use crate::stores::resource::Resource;
use crate::stores::store::Store;

pub fn load() -> Result<Vec<Store>, Box<dyn Error + Send + Sync>> {
    let resource: EmbeddedFile = Resource::get("stores.json").unwrap();
    let string = std::str::from_utf8(resource.data.as_ref()).unwrap();

    let stores: Vec<Store> = serde_json::from_str(string)?;

    Ok(stores)
}
