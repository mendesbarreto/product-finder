use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "resource/"]
pub struct Resource;
