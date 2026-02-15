use std::{collections::HashMap, fmt::format};

#[derive(Clone, Debug)]
pub enum VfsNode {
    File { name: String, content: String },
    Directory { name: String, children: Vec<String> },
}

#[derive(Clone, Debug)]
pub struct VirtualFs {
    pub nodes: HashMap<String, VfsNode>,
}

impl VirtualFs {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_dir(&mut self, path: &str, children: &[&str]) {
        let name = path.rsplit('/').next().unwrap_or(path).to_string();
        self.nodes.insert(
            path.to_string(),
            VfsNode::Directory {
                name,
                children: children.iter().map(|s| s.to_string()).collect(),
            },
        );
    }

    pub fn add_file(&mut self, path: &str, content: &str) {
        let name = path.rsplit('/').next().unwrap_or(path).to_string();
        self.nodes.insert(
            path.to_string(),
            VfsNode::File {
                name,
                content: content.to_string(),
            },
        );
    }

    /// Resolve a path relative to cwd. Handles "~", "..", ".", and absolute paths.
    pub fn resolve_path(&self, cwd: &str, input: &str) -> String {
        let input = input.trim();

        // handle absolute paths
        if input.starts_with("~/") || input == "~" {
            return self.normalize(input);
        }

        // Handle relative paths
        let combined = if cwd == "~" {
            format!("~/{}", input)
        } else {
            format!("{}/{}", cwd, input)
        };

        self.normalize(&combined)
    }

    /// Normalize a path: resolve ".." and ".", remove trailing slashes
    fn normalize(&self, path: &str) -> String {
        let mut parts: Vec<&str> = Vec::new();

        for part in path.split('/') {
            match part {
                "." | "" => continue,
                ".." => {
                    if parts.len() > 1 || (parts.len() == 1 && parts[0] != "~") {
                        parts.pop();
                    }
                }
                other => parts.push(other),
            }
        }

        if parts.is_empty() {
            "~".to_string()
        } else {
            parts.join("/")
        }
    }

    pub fn is_dir(&self, path: &str) -> bool {
        matches!(self.nodes.get(path), Some(VfsNode::Directory { .. }))
    }

    pub fn is_file(&self, path: &str) -> bool {
        matches!(self.nodes.get(path), Some(VfsNode::File { .. }))
    }

    pub fn ls(&self, path: &str) -> Option<Vec<(String, bool)>> {
        match self.nodes.get(path) {
            Some(VfsNode::Directory { children, .. }) => {
                let mut entries: Vec<(String, bool)> = children
                    .iter()
                    .map(|child| {
                        let child_path = if path == "~" {
                            format!("~/{}", child)
                        } else {
                            format!("{}{}", path, child)
                        };
                        let is_dir = self.is_dir(&child_path);
                        (child.clone(), is_dir)
                    })
                    .collect();
                entries.sort_by(|a, b| a.0.cmp(&b.0));
                Some(entries)
            }
            _ => None,
        }
    }

    pub fn cat(&self, path: &str) -> Option<&str> {
        match self.nodes.get(path) {
            Some(VfsNode::File { content, .. }) => Some(content),
            _ => None,
        }
    }

    pub fn tree(&self, path: &str, prefix: &str, is_last: bool) -> Option<Vec<String>> {
        let node = self.nodes.get(path)?; // return null immediately if this is None.
        let mut lines = Vec::new();

        let name = match node {
            VfsNode::File { name, .. } => name.clone(),
            VfsNode::Directory { name, .. } => format!("{}/", name),
        };

        if prefix.is_empty() {
            // Root of tree call
            lines.push(name);
        } else {
            let connector = if is_last { "└── " } else { "├── " };
            lines.push(format!("{}{}{}", prefix, connector, name));
        }

        if let VfsNode::Directory { children, .. } = node {
            let child_prefix = if prefix.is_empty() {
                String::new()
            } else if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            for (i, child) in children.iter().enumerate() {
                let child_path = if path == "~" {
                    format!("~/{}", child)
                } else {
                    format!("{}/{}", path, child)
                };
                let is_last_child = i == children.len() - 1;
                if let Some(child_lines) = self.tree(&child_path, &child_prefix, is_last_child) {
                    lines.extend(child_lines);
                }
            }
        }

        Some(lines)
    }
}
