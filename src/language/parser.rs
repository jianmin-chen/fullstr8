use crate::language::lexer::{Token, TokenType};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum NodeType {
  // NodeType is a limited subset of TokenType
  // If these had types attached to them, they would be more useful in the interpreter
  // But that's ok
  Card,
  Number
}

#[derive(Debug, Serialize)]
pub struct Node {
  pub args: Vec<Node>,
  pub kind: NodeType
}

impl Node {
  pub fn new(caller: String, args: Vec<Node>, kind: NodeType) -> Self {
    Self {
      caller,
      args,
      kind
    }
  }
}

pub struct Parser {
  current: usize,
  tokens: Vec<Token>,
  pub ast: Vec<Node>
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self {
      current: 0,
      tokens,
      ast: Vec::new()
    }
  }

  pub fn peek_type(&self) -> &TokenType {
    &self.tokens[self.current].kind
  }

  pub fn peek(&self) -> &Token {
    &self.tokens[self.current]
  }

  pub fn eat(&mut self, keyword: TokenType) -> &Token {
    if *self.peek_type() == keyword {
      self.current += 1;
      &self.tokens[self.current - 1]
    } else {
      panic!("Expected {:?} but got {:?}", keyword, self.peek_type());
    }
  }

  pub fn call(&mut self) -> Node {
    self.eat(TokenType::LeftParen);
    let kind = self.peek_type();
    match *kind {
      TokenType::Card => {
        let caller = self.eat(TokenType::Card);
        let value = caller.value.clone();
        if *self.peek_type() == TokenType::RightParen {
          // No arguments
          self.eat(TokenType::RightParen);
          return Node::new(value, Vec::new(), NodeType::Card);
        } else {
          // Read arguments
          let mut callee: Vec<Node> = Vec::new();
          while *self.peek_type() != TokenType::RightParen {
            let node = self.call();
            callee.push(node);
          }
          self.eat(TokenType::RightParen);
          return Node::new(value, callee, NodeType::Card);
        }
      },
      TokenType::Number => {
        let number = self.eat(TokenType::Number).value.clone();
        self.eat(TokenType::RightParen);
        return Node::new(number, Vec::new(), NodeType::Number);
      },
      _ => {
        panic!("Unexpected token of type {:?}", kind);
      }
    }
  }
  
  pub fn parse(&mut self) {
    while *self.peek_type() != TokenType::Eof {
      let node = self.call();
      self.ast.push(node);
    }
  }
}