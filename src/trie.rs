/// https://dev.to/timclicks/two-trie-implementations-in-rust-ones-super-fast-2f3m
use std::{collections::HashMap, fmt::Display};

#[derive(Default, Debug, PartialEq)]
enum TrieNodeEnd {
    #[default]
    Continue,
    Absolute,
    Wildcard,
}

#[derive(Default, Debug)]
struct TrieNode {
    end: TrieNodeEnd,
    children: HashMap<char, TrieNode>,
}

#[derive(Default, Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, chars: &str) {
        if chars.len() == 0 {
            return;
        }

        let end = if chars.chars().nth(0).unwrap() == '.' {
            TrieNodeEnd::Wildcard
        } else {
            TrieNodeEnd::Absolute
        };

        let mut current_node = &mut self.root;
        let mut parent_node_end = None;
        for c in chars.chars().rev() {
            parent_node_end = Some(&mut current_node.end);
            current_node = current_node.children.entry(c).or_default();

            if c == '.' && current_node.end == TrieNodeEnd::Wildcard {
                return;
            }
        }

        let has_wildcard = current_node
            .children
            .get(&'.')
            .is_some_and(|n| n.end == TrieNodeEnd::Wildcard);

        // Don't add the absolute version if the wildcard version exists.
        if has_wildcard && current_node.end == TrieNodeEnd::Absolute {
            return;
        }

        // If this node is a wildcard, and parent is absolute, defer to the wildcard.
        if let Some(parent_end) = parent_node_end {
            if end == TrieNodeEnd::Wildcard && *parent_end == TrieNodeEnd::Absolute {
                *parent_end = TrieNodeEnd::Continue;
            }
        }

        // Remove child nodes.
        if end == TrieNodeEnd::Wildcard {
            current_node.children = HashMap::new();
        }

        current_node.end = end;
    }

    pub fn matches<'a>(&self, word: &'a str) -> bool {
        let mut current_node = &self.root;

        for c in word.chars().rev() {
            match current_node.children.get(&c) {
                Some(node) => {
                    current_node = node;

                    if node.end == TrieNodeEnd::Wildcard {
                        return true;
                    }
                }
                None => {
                    return false;
                }
            }
        }

        if let Some(node) = current_node.children.get(&'.') {
            if node.end == TrieNodeEnd::Wildcard {
                return true;
            }
        }

        current_node.end != TrieNodeEnd::Continue
    }
}

impl Display for Trie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (c, root_children) in &self.root.children {
            let mut node_stack = vec![root_children];
            let mut string_stack = format!("{}", c);

            print_children(&mut node_stack, &mut string_stack, f)?;
        }

        Ok(())
    }
}

fn print_children(
    node_stack: &mut Vec<&TrieNode>,
    string_stack: &mut String,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    let head = node_stack.last().unwrap();

    if head.end != TrieNodeEnd::Continue {
        writeln!(f, "{}", string_stack.chars().rev().collect::<String>())?;
    }

    for (c, child) in &head.children {
        node_stack.push(&child);
        string_stack.push(*c);

        print_children(node_stack, string_stack, f)?;

        string_stack.pop();
        node_stack.pop();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches() {
        let mut trie = Trie::new();

        trie.insert("bar.com");
        trie.insert(".c.bar.com");
        trie.insert("a.bar.com");
        trie.insert("aa.bar.com");
        trie.insert("b.bar.com");
        trie.insert("b.bar.com");
        trie.insert("c.bar.com");
        trie.insert(".cc.bar.com");

        assert!(trie.matches("thing.dong.cc.bar.com"));
        assert!(trie.matches(".dong.cc.bar.com"));
        assert!(trie.matches("dong.cc.bar.com"));
        assert!(trie.matches("c.bar.com"));
        assert!(trie.matches("cc.bar.com"));

        assert!(!trie.matches("bb.bar.com"));

        assert!(!trie.matches("d.bar.com"));
        trie.insert(".bar.com");
        assert!(trie.matches("d.bar.com"));
    }
}
