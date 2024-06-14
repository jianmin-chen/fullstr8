use crate::language::parser::{Node, NodeType};

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

  pub fn evaluate(&mut self, node: &Node) {
    match node.kind {
      NodeType::Number => {},
      _ => {
        panic!("Unexpected type")
      }
    }
  }

  pub fn execute(&mut self, node: &Node) {
    match node.kind {
      NodeType::Card => {
        _ => {
          
        } 
      },
      NodeType::Number => {
        self.evaluate(node)
      }   
    }
  }

  pub fn interpret(&mut self, ast: Vec<Node>) {
    for node in &ast {
      self.execute(node);
    }
  }
}