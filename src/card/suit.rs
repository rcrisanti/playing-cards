use colored::Colorize;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let emoji = match self {
            Self::Clubs => "\u{2663}".black(),
            Self::Spades => "\u{2660}".black(),
            Self::Hearts => "\u{2665}".red(),
            Self::Diamonds => "\u{2666}".red(),
        };
        write!(f, "{}", emoji.on_bright_white())
    }
}
