extern crate rand;

use self::rand::{thread_rng, Rng};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CardFace {
  TWO,
  THREE,
  FOUR,
  FIVE,
  SIX,
  SEVEN,
  EIGHT,
  NINE,
  TEN,
  JACK,
  QUEEN,
  KING,
  ACE,
  JOKER
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CardSuite {
    SPADES,
    CLUB,
    HEARTH,
    DIAMOND
}

pub type Card = (CardFace, Option<CardSuite>);

use self::CardFace::*;
use self::CardSuite::*;

pub const CARDS: [Card;54] = [ 
    (JOKER,None), (JOKER,None),
    (TWO,Some(SPADES)), (THREE,Some(SPADES)), (FOUR,Some(SPADES)),
    (FIVE,Some(SPADES)), (SIX,Some(SPADES)), (SEVEN,Some(SPADES)), (EIGHT,Some(SPADES)),
    (NINE,Some(SPADES)), (TEN,Some(SPADES)), (JACK,Some(SPADES)), (QUEEN,Some(SPADES)),
    (KING,Some(SPADES)), (ACE,Some(SPADES)),
    (TWO,Some(CLUB)), (THREE,Some(CLUB)), (FOUR,Some(CLUB)),
    (FIVE,Some(CLUB)), (SIX,Some(CLUB)), (SEVEN,Some(CLUB)), (EIGHT,Some(CLUB)),
    (NINE,Some(CLUB)), (TEN,Some(CLUB)), (JACK,Some(CLUB)), (QUEEN,Some(CLUB)),
    (KING,Some(CLUB)), (ACE,Some(CLUB)),
    (TWO,Some(HEARTH)), (THREE,Some(HEARTH)), (FOUR,Some(HEARTH)),
    (FIVE,Some(HEARTH)), (SIX,Some(HEARTH)), (SEVEN,Some(HEARTH)), (EIGHT,Some(HEARTH)),
    (NINE,Some(HEARTH)), (TEN,Some(HEARTH)), (JACK,Some(HEARTH)), (QUEEN,Some(HEARTH)),
    (KING,Some(HEARTH)), (ACE,Some(HEARTH)),
    (TWO,Some(DIAMOND)), (THREE,Some(DIAMOND)), (FOUR,Some(DIAMOND)),
    (FIVE,Some(DIAMOND)), (SIX,Some(DIAMOND)), (SEVEN,Some(DIAMOND)), (EIGHT,Some(DIAMOND)),
    (NINE,Some(DIAMOND)), (TEN,Some(DIAMOND)), (JACK,Some(DIAMOND)), (QUEEN,Some(DIAMOND)),
    (KING,Some(DIAMOND)), (ACE,Some(DIAMOND)),
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Self {
        Deck{ cards: CARDS.to_vec() }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();

        let mut cards_sorter: Vec<_> = self.cards.iter()
            .map(|c| (rng.gen::<u32>(),c.clone()))
            .collect();

        cards_sorter.sort_by_key(|&(c,_)| c);

        self.cards = cards_sorter.iter().map(|&(_,v)| v).collect();
    }

    pub fn deal_one_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}