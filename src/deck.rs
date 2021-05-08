use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
  Clubs,
  Diamonds,
  Hearts,
  Spades
}

pub type Rank = usize;

#[derive(Clone, PartialEq, Eq)]
pub struct Card {
  pub rank: Rank,
  pub suit: Suit,
}

impl fmt::Display for Card {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut ret = String::from("");
    ret.push(match self.rank {
      2..=9 => ('0' as u8 + self.rank as u8) as char,
      10 => 'T',
      11 => 'J',
      12 => 'Q',
      13 => 'K',
      14 => 'A',
      _ => panic!("Rank is invalid!"),
    });
    ret.push(match self.suit {
      Suit::Clubs => 'c',
      Suit::Diamonds => 'd',
      Suit::Hearts => 'h',
      Suit::Spades => 's',
    });
    write!(f, "{}", ret)
  }
}

impl From<u8> for Card {
  fn from(inp: u8) -> Card {
    if inp >= 53 {
      panic!("Tried to create Card with integer {}!", inp);
    }
    return Card {
      rank: 2 + (inp as usize) / 4, // Range from 2->14, 14 is Ace
      suit: match inp % 4 {
        0 => Suit::Clubs, 1 => Suit::Diamonds, 2 => Suit::Hearts, 3 => Suit::Spades,
        _ => panic!("Unreachable code!"),
      }
    };
  }
}

use rand::Rng;

pub struct Deck {
  cards: Vec<Card>,
  cur_index: usize,
}

impl Deck {
  pub fn new() -> Self {
    let mut ret = Deck { cards: vec![], cur_index: 52 };
    for i in 0..52 {
      ret.cards.push(Card::from(i));
    }
    return ret;
  }
  pub fn shuffle(&mut self) {
    let mut rng = rand::thread_rng();
    for i in 0..51 {
      let other = rng.gen_range((i+1)..52);
      self.cards.swap(i, other);
    }
    self.cur_index = 52;
  }
  pub fn draw_card(&mut self) -> Card {
    self.cur_index -= 1;
    self.cards[self.cur_index].clone()
  }
}

impl fmt::Display for Deck {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for i in &self.cards {
      write!(f, "{} ", i)?
    }
    Ok(())
  }
}
