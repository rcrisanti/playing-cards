use crate::{Card, Suit};
use itertools::Itertools;
use rand::{
    prelude::{SliceRandom, ThreadRng},
    thread_rng,
};
use std::fmt::Display;

pub trait Deck {
    fn size(&self) -> usize;

    fn shuffle(&mut self);

    fn draw(&mut self) -> Option<Card>;

    fn insert(&mut self, card: Card, pos: usize);

    fn cut(&mut self, pos: usize) -> Result<Card, &str>;

    fn contains(&self, card: &Card) -> bool;

    fn peek(&self, pos: usize) -> Option<&Card>;
}

#[derive(Debug, Clone)]
pub struct StandardDeck {
    cards: Vec<Card>,
    rng: ThreadRng,
}

impl StandardDeck {
    pub fn new(jokers: usize) -> Self {
        let mut cards = (1..14)
            .cartesian_product(vec![
                Suit::Hearts,
                Suit::Clubs,
                Suit::Diamonds,
                Suit::Spades,
            ])
            .map(|(value, suit)| Card::new(value, Some(suit)).expect("could not build card"))
            .collect::<Vec<_>>();

        for _ in 0..jokers {
            let joker = Card::new(0, None).expect("could not build joker");
            cards.push(joker.clone());
        }

        StandardDeck {
            cards,
            rng: thread_rng(),
        }
    }
}

impl Deck for StandardDeck {
    fn size(&self) -> usize {
        self.cards.len()
    }

    fn shuffle(&mut self) {
        self.cards.shuffle(&mut self.rng);
    }

    fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    fn insert(&mut self, card: Card, pos: usize) {
        self.cards.insert(pos, card)
    }

    fn cut(&mut self, pos: usize) -> Result<Card, &str> {
        if pos >= self.size() {
            return Err("cut position out of bounds for current deck size");
        } else {
            return Ok(self.cards.remove(pos));
        }
    }

    fn contains(&self, card: &Card) -> bool {
        self.cards.contains(card)
    }

    fn peek(&self, pos: usize) -> Option<&Card> {
        self.cards.get(pos)
    }
}

impl Display for StandardDeck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.cards
                .iter()
                .map(|l| { format!("{} ", l) })
                .collect::<String>()
        )
    }
}
