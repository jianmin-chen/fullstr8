use crate::deck::deck;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum TokenType {
  LeftParen, RightParen, Card, Number, Eof
}

#[derive(Debug, Serialize)]
pub struct Token {
  pub kind: TokenType,
  pub value: String
}

impl Token {
  pub fn new(kind: TokenType, value: String) -> Self {
    Self {
      kind,
      value
    }
  }
}

pub struct Lexer {
  current: usize,
  program: String,
  pub tokens: Vec<Token>,
  deck: Vec<String>
}

impl Lexer {
  pub fn new(program: String) -> Self {
    Self {
      current: 0,
      program,
      tokens: Vec::new(),
      deck: deck()
    }
  }

  pub fn peek(&self) -> char {
    if self.current + 1 > self.program.len() {
      return '\0'
    }
    self.program.chars().nth(self.current).unwrap()
  }

  pub fn advance(&mut self) -> char{
    if self.current + 1 > self.program.len() {
      return '\0'
    }
    self.current += 1;
    self.program.chars().nth(self.current - 1).unwrap()
  }

  pub fn r#match(&mut self, chr: char) -> bool {
    if self.peek() == chr {
      self.advance();
      return true
    }
    false
  }

  pub fn card(&self, kw: &String) -> bool { 
    if self.deck.contains(kw) == true {
      return true;
    }
    false
  }

  pub fn scan_token(&mut self) {
    let chr = self.advance();
    let repr = String::from(chr);
    let token: Option<Token> = match chr {
      '(' => Some(Token::new(TokenType::LeftParen, repr)),
      ')' => Some(Token::new(TokenType::RightParen, repr)),
      '-' => {
        while self.peek() != '\n' && self.peek() != '\0' {
          self.advance();
        }
        None
      },
      ' ' | '\n' | '\t' | '\r' => None,
      _ => {
        // Check if it's a number or a card, and panic otherwise
        // TODO: Take care of numeric digits in cards, like 1 of clubs, etc.
        if chr.is_digit(10) {
          let mut digits: String = String::from(chr);
          while self.peek().is_digit(10) {
            digits += &self.advance().to_string();
          }
          Some(Token::new(TokenType::Number, digits))
        } else {
          let mut kw: String = String::from(chr);
          loop {
            let next_chr = self.advance();
            if next_chr == '\0' {
              panic!("{} is not a valid card", kw);
            } 
            kw += &next_chr.to_string();
            if self.card(&kw) == true {
              break;
            }
          }
          Some(Token::new(TokenType::Card, kw))
        }
      }
    };
    if let Some(tok) = token {
      self.tokens.push(tok);
    }
  }

  pub fn scan(&mut self) {
    while self.peek() != '\0' {
      self.scan_token();
    }
    self.tokens.push(Token::new(TokenType::Eof, String::from("EOF")));
  }
}