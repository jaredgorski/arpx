#[derive(Clone, Debug)]
pub struct RollingBuffer {
    pub lines: Vec<String>,
    pub size: usize,
}

impl RollingBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            lines: Vec::new(),
            size,
        }
    }

    pub fn push(&mut self, line: String) {
        if self.lines.len() == self.size {
            self.lines.remove(0);
        }

        self.lines.push(line);
    }

    pub fn dump(&self) -> String {
        self.lines.join("\n")
    }
}
