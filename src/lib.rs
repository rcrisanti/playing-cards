pub mod card;
pub mod deck;

pub use card::card::Card;
pub use card::suit::Suit;
pub use deck::{Deck, StandardDeck};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
