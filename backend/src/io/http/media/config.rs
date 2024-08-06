

pub const REL_MEDIA_LOCATION: &str = "assets";

pub fn pad_url_media_location(file_name : &String) -> String {
    format!("/{}/{}", REL_MEDIA_LOCATION, file_name)
}