use playing_cards::{Card, Deck, StandardDeck, Suit};

#[main]
fn main() {
    let mut deck = StandardDeck::new(2);
    println!("{}", deck.size());
    println!("{}", deck);
    deck.shuffle();
    println!("{}", deck);

    // while let Some(card) = deck.draw() {
    //     println!("{}", card);
    // }

    // println!("{:?}", deck);
}
