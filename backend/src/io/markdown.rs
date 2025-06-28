pub fn markdown_link(name: &String, destination: &String) -> String {
    format!("[{name}]({destination})")
}
