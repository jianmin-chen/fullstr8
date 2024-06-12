use crate::language::lexer::{Token, TokenType};

pub struct Node {
  pub caller: String,
  pub args: Vec<Node>,
}

impl Node {
  pub fn new(caller: String, args: Vec<Node>) -> Self {
    Self {
      caller,
      args,
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

  pub fn peek_type(&self) -> Option<&TokenType> {
    if self.current + 1 >= self.tokens.len() {
      return None;
    }
    Some(&self.tokens[self.current].kind)
  }

  pub fn peek(&self) -> Option<&Token> {
    if self.current + 1 >= self.tokens.len() {
      return None;
    }
    Some(&self.tokens[self.current])
  }

  pub fn advance(&mut self) -> Option<&Token> {
    if self.current + 1 >= self.tokens.len() {
      return None;
    }
    self.current += 1;
    Some(&self.tokens[self.current])
  }

  pub fn eat(&mut self, keyword: TokenType) -> Option<&Token> {
    if let Some(kind) = self.peek_type() {
      if *kind == keyword {
        Some(self.advance().unwrap())
      } else {
        panic!("Expected {:?} but got {:?}", *kind, keyword);
      }
    } else {
      panic!("Unexpected end of file");
    }
  }

  pub fn call(&mut self) -> Node {
    self.eat(TokenType::LeftParen);
    let kind = self.peek_type().unwrap();
    match *kind {
      TokenType::Card => {
        let caller = self.eat(TokenType::Card).unwrap();
        let value = caller.value.clone();
        if *self.peek_type().unwrap() == TokenType::RightParen {
          // No arguments
          self.eat(TokenType::RightParen);
          return Node::new(value, Vec::new());
        } else {
          // Read arguments
          let mut callee: Vec<Node> = Vec::new();
          while *self.peek_type().unwrap() != TokenType::RightParen {
            callee.push(self.call());
          }
          return Node::new(value, callee);
        }
      },
      TokenType::Number => {
        panic!("Working on it!");
      },
      _ => {
        panic!("Unexpected token of type {:?}", kind);
      }
    }
  }
  
  pub fn parse(&mut self) {
    while let Some(kind) = self.peek_type() {
    }
  }
}