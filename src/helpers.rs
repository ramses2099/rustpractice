pub mod namehelpers {
    pub fn get_full_name(first_name: &str, last_name: &str) -> String {
        let full_name: String = format!("{0} {1}", first_name, last_name);
        full_name
    }
}
