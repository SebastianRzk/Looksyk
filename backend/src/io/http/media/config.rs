use crate::looksyk::syntax::markdown::encode_uri_component;

pub const REL_MEDIA_LOCATION: &str = "assets";

pub fn create_media_location(file_name : &String) -> String {
    format!("/{}/{}", REL_MEDIA_LOCATION, encode_uri_component(file_name))
}