use axum;
use std::fs;
use std::env;
use std::io;
use std::io::Write;
use serde_json::json;

pub mod language;
pub mod server;

use language::lexer::Lexer;
use server::server::router;

#[tokio::main]
async fn main() {
  let mut args: Vec<String> = env::args().skip(1).collect();

  let mut dbg = false;
  let index = args.iter().position(|arg| arg == "--dbg");
  if let Some(idx) = index {
    dbg = true;
    args.remove(idx);
  }

  if args.len() == 1 {
    let program = fs::read_to_string(&args[0]).unwrap();
    let mut lexer = Lexer::new(program);
    lexer.scan();

    if dbg == true {
      let dbg_lexer = json!(lexer.tokens);
      fs::write("lexer.json", dbg_lexer.to_string()).expect("Unable to write debugger output for lexer");
    }
  } else {  
    let mut stdin = io::stdin();
    let input = &mut String::new();

    loop {
      input.clear();
      print!("> ");
      io::stdout().flush().unwrap();
      stdin.read_line(input);
      dbg!(&input);
    }
  }

  println!("Listening on port 3000");
  let app = router();
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}