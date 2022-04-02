use super::suit::Suit;
use colored::Colorize;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Card {
    value: u8,
    suit: Option<Suit>,
}

impl Card {
    pub fn new(value: u8, suit: Option<Suit>) -> Result<Self, CardError> {
        if value == 0 && suit.is_some() {
            return Err(CardError::JokerHasSuit);
        } else if value > 13 {
            return Err(CardError::ValueTooLarge);
        } else if value > 0 && suit.is_none() {
            return Err(CardError::NoSuit);
        }

        Ok(Card { value, suit })
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn suit(&self) -> Option<Suit> {
        self.suit
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        if let Some(suit) = self.suit {
            if let Some(osuit) = other.suit {
                return suit == osuit && self.value == other.value;
            }
        } else {
            if other.suit.is_none() {
                return self.value == other.value;
            }
        }
        false
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = match self.value() {
            0 => format!(
                "{}{}{}",
                "\u{1F921}".black().on_bright_white(),
                "Jo".black().on_bright_white(),
                "\u{1F921}".black().on_bright_white()
            ),
            _ => {
                let suit = self
                    .suit
                    .expect("should be a suit for every card except Jokers");
                format!(
                    "{}{}{}",
                    suit,
                    match self.value {
                        1 => "A".to_owned(),
                        11 => "J".to_owned(),
                        12 => "Q".to_owned(),
                        13 => "K".to_owned(),
                        _ => self.value().to_string(),
                    }
                    .black()
                    .on_bright_white(),
                    suit
                )
            }
        };

        write!(f, "{}", display_str)
    }
}

#[derive(Debug)]
pub enum CardError {
    JokerHasSuit,
    ValueTooLarge,
    NoSuit,
}

impl Display for CardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CardError")
    }
}

impl Error for CardError {}
