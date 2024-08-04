use super::ParseFile;

pub struct AdblockPlus {}

impl ParseFile for AdblockPlus {
    fn parse_file(&self, file: &str) -> Vec<String> {
        file.lines()
            .skip(1)
            // This is mostly what I need to support for now.
            .filter(|line| line.starts_with("||") && line.ends_with("^"))
            .map(|line| &line[2..line.len() - 1])
            .map(String::from)
            .map(|entry| format!(".{}", entry))
            .collect()
    }
}
