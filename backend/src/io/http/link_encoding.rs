pub fn encode_link_component(link: &String) -> String {
    link.replace(" ", "%20").replace("#", "%23").replace("/", "%2F")
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_encode_link_component() {
        assert_eq!(super::encode_link_component(&"a b".to_string()), "a%20b");
        assert_eq!(super::encode_link_component(&"a#b".to_string()), "a%23b");
        assert_eq!(super::encode_link_component(&"a/b".to_string()), "a%2Fb");
    }
}