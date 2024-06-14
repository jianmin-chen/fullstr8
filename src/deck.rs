pub fn deck() -> Vec<String> {
  // Generate all cards in a deck
  let suits = vec!["clubs", "diamonds", "hearts", "spades"];
  let mut cards: Vec<String> = Vec::new();
  for suit in &suits {
    for n in 2..11 {
      cards.push(n.to_string() + " of " + suit);
    }
    for n in vec!["two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"] {
      cards.push(String::from(n) + " of " + suit);
    }
    cards.push("ace of ".to_string() + suit);
    cards.push("jack of ".to_string() + suit);
    cards.push("queen of ".to_string() + suit);
    cards.push("king of ".to_string() + suit);
  }
  cards.push(String::from("joker"));
  cards
}