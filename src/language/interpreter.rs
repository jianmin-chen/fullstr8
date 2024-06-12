use crate::language::parser::Node;

pub struct Interpreter {
  index: usize,
  memory: Vec<usize>
}

impl Interpreter {
  pub fn new(memory: Vec<usize>, index: usize) -> Self {
    Self {
      index,
      memory
    }
  }

  pub fn execute(&mut self, node: &Node) {
  }

  pub fn interpret(&mut self, ast: Vec<Node>) {
    for node in &ast {
      self.execute(node);
    }
  }
}