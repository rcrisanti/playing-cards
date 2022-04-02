use crate::{Card, Suit};

use super::*;

#[test]
fn test_shuffle() {
    let mut deck = StandardDeck::new(0);
    let deck2 = deck.clone();
    deck.shuffle();
    assert_eq!(deck.size(), deck2.size(), "deck changed size on shuffle");

    while let Some(card) = deck.draw() {
        assert!(deck2.contains(&card))
    }
}

#[test]
fn test_draw() {
    let mut deck = StandardDeck::new(0);
    let spade_king = Card::new(13, Some(Suit::Spades)).expect("could not build card");
    assert_eq!(spade_king, deck.draw().expect("unable to draw card"));
}

#[test]
fn test_shuffle_draw() {
    let mut deck = StandardDeck::new(0);
    deck.shuffle();
    let top_card = deck.peek(51).expect("not enough cards in deck").clone();
    assert_eq!(top_card, deck.draw().expect("unable to draw card"));
}

#[test]
fn test_insert() {
    let mut deck = StandardDeck::new(0);
    let new_card = Card::new(0, None).expect("could not build card");
    let old_12 = deck.peek(12).expect("no 13th card").clone();
    let old_13 = deck.peek(13).expect("no 14th card").clone();
    deck.insert(new_card, 13);
    assert_eq!(deck.peek(12).expect("no 13th card").clone(), old_12);
    assert_eq!(deck.peek(13).expect("no 14th card").clone(), new_card);
    assert_eq!(deck.peek(14).expect("no 15th card").clone(), old_13);
}

#[test]
fn test_cut() {
    let mut deck = StandardDeck::new(0);
    let old_12 = deck.peek(12).expect("no 13th card").clone();
    let old_14 = deck.peek(14).expect("no 15th card").clone();
    let _cut_card = deck.cut(13).expect("no 14th card");
    assert_eq!(deck.peek(12).expect("no 13th card").clone(), old_12);
    assert_eq!(deck.peek(13).expect("no 14th card").clone(), old_14);
}

#[test]
fn test_peek() {
    let deck = StandardDeck::new(0);
    assert_eq!(
        deck.peek(0).expect("no first card").clone(),
        Card::new(1, Some(Suit::Hearts)).expect("could not build card")
    );
    assert_eq!(
        deck.peek(51).expect("no first card").clone(),
        Card::new(13, Some(Suit::Spades)).expect("could not build card")
    );
}

#[test]
fn test_contains() {
    let deck = StandardDeck::new(0);
    assert!(deck.contains(&Card::new(8, Some(Suit::Spades)).expect("could not build card")));
    assert!(!deck.contains(&Card::new(0, None).expect("could not build card")));
}
