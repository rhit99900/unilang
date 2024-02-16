pub struct SourceText {
  text: String
}

impl SourceText {
  pub fn new(text: String) -> Self {
    return Self { text };
  }

  pub fn line_index(&self, position: usize) -> usize {
    self.text[..position].lines().count() - 1
  }

  pub fn get_line(&self, index: usize) -> &str {
    return self.text.lines().nth(index).unwrap();
  }

  pub fn line_start(&self, index: usize) -> usize {
    return self.text.lines().take(index).map(|line| line.len() + 1).sum();
  }
}