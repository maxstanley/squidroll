use super::ParseFile;

pub struct DomainList {}

impl ParseFile for DomainList {
    fn parse_file(&self, file: &str) -> Vec<String> {
        file.lines()
            .filter(|line| !line.starts_with("#"))
            .map(String::from)
            .collect()
    }
}
