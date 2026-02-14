pub struct ParsedCommand {
  pub command: String,
  pub args: Vec<String>,
}

pub fn parse(input: &str) -> ParsedCommand {
  let trimmed = input.trim();
  let parts: Vec<&str> = trimmed.split_whitespace().collect();

  match parts.first() {
    Some(cmd) => ParsedCommand {
      command: cmd.to_lowercase(),
      args: parts[1..].iter().map(|s| s.to_string()).collect(),
    },
    None => ParsedCommand {
      command: String::new(),
      args: vec![],
    },
  }
}