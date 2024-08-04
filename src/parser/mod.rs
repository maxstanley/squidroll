pub mod adblockplus;
pub mod domainlist;

pub trait ParseFile {
    fn parse_file(&self, file: &str) -> Vec<String>;
}
