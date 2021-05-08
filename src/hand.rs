use crate::deck;
use deck::Card;
use deck::Deck;
use deck::Suit;
use deck::Rank;

use std::collections::HashMap;
macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::array::IntoIter::new([$(($k, $v),)*]))
    };
    // set-like
    ($($v:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::array::IntoIter::new([$($v,)*]))
    };
}

#[derive(Debug)]
pub enum HandRank {
  HighCard,
  Pair,
  TwoPair,
  Trips,
  Straight,
  Flush,
  FullHouse,
  Quads,
  StraightFlush,
}

use HandRank::*;
use Suit::*;

pub struct FinalHand {
  pub hand_rank: HandRank,
  pub hand: Vec<Card>,
}

// primary_hand consists of just the cards used in the HandRank, total_hand consists of all cards.
// This function returns HandRank cards + grabs kickers from total_hand
fn best_five(mut primary_hand: Vec<Card>, total_hand: Vec<Card>) -> Vec<Card> {
    // Get cards that aren't in the primary hand
    let mut remaining_hand: Vec<Card> = total_hand.into_iter().filter(|c| -> bool { !primary_hand.contains(c) }).collect();
    // Sort by rank
    remaining_hand.sort_by(|a, b| b.rank.cmp(&a.rank));
    if primary_hand.len() < 5 {
        primary_hand.extend_from_slice(&remaining_hand[0..5-primary_hand.len()]);
    }
    return primary_hand;
}

pub fn best_five_of_seven(param_total_hand: Vec<Card>) -> FinalHand {
    let mut cards_by_rank: Vec<Vec<Suit>>  = vec![vec![]; 15];
    let mut cards_by_suit: HashMap<Suit,Vec<Rank>> = collection! {
        Clubs => vec![], Diamonds => vec![], Hearts => vec![], Spades => vec![],
    };
    let mut total_hand = param_total_hand.clone();
    total_hand.sort_by(|a, b| b.rank.cmp(&a.rank));
    for c in &total_hand {
        cards_by_rank.get_mut(c.rank).unwrap().push(c.suit);
        cards_by_suit.get_mut(&c.suit).unwrap().push(c.rank);
    }

    // Check for Flush
    let mut has_flush = false;
    let mut flush_hand: Vec<Card> = vec![];
    for c in [Clubs, Diamonds, Hearts, Spades] {
        let flush = &mut cards_by_suit.get_mut(&c).unwrap();
        if flush.len() >= 5 {
            // Flush found! Now sort from high-to-low
            flush.sort_by(|a, b| b.cmp(a));
            flush_hand = flush[0..5].into_iter().map(|rank| -> Card { return Card { rank: *rank, suit: c } }).collect();

            // Check for the best Straight Flush inside of the flush
            for i in 0..(flush.len() - 5) {
                let mut works = true;
                for j in 1..5 {
                    if flush[j-1] - flush[j] != 1 {
                        works = false;
                    }
                }
                if (i+1..i+5).into_iter().all(|j| flush[j-1] - flush[j] == 1) {
                    return FinalHand {
                        hand_rank: StraightFlush,
                        hand: flush[i..i+5].into_iter().map(|rank| -> Card { return Card { rank: *rank, suit: c } }).collect(),
                    }
                }
            }

            // We're done, there can only be one flush, since 5/7 are already of a single suit
            return FinalHand {
                hand_rank: Flush,
                hand: flush_hand,
            };
        }
    }

    // Get pairs and trips, return early for quads
    let mut pairs: Vec<Rank> = vec![];
    let mut trips: Vec<Rank> = vec![];
    for i in (2..=14).rev() { // rev so higher pairs/trips come first
        match cards_by_rank[i].len() {
            0..=1 => (),
            2 => pairs.push(i),
            3 => trips.push(i),
            4 => return FinalHand {
                hand_rank: Quads,
                hand: best_five(total_hand.clone().into_iter().filter(|c| -> bool {c.rank == i}).collect(), total_hand),
            },
            _ => panic!("5 pair or more is not possible! Found with index {}", i),
        }
    }

    // Check for full house from pairs and trips
    if pairs.len() > 0 && trips.len() > 0 {
        return FinalHand {
            hand_rank: FullHouse,
            hand: total_hand.clone().into_iter().filter(|c| -> bool {c.rank == pairs[0] || c.rank == trips[0]}).collect(),
        };
    }
    // Check for full house from two trips
    if trips.len() > 1 {
        let mut ret: Vec<Card> = total_hand.into_iter().filter(|c| -> bool {c.rank == trips[0] || c.rank == trips[1]}).collect();
        ret.pop().unwrap(); // Remove the unneeded card from the 2nd trip
        return FinalHand {
            hand_rank: FullHouse,
            hand: ret,
        };
    }

    // Check for Straight
    for i in 0..(total_hand.len() - 5) {
        if (i+1..i+5).into_iter().all(|j| total_hand[j-1].rank - total_hand[j].rank == 1) {
            return FinalHand {
                hand_rank: Straight,
                hand: (&total_hand[i..(i+5)]).to_vec(),
            };
        }
    }

    // Check for trips
    if trips.len() > 0 {
        return FinalHand {
            hand_rank: Trips,
            hand: best_five(total_hand.clone().into_iter().filter(|c| -> bool {c.rank == trips[0]}).collect(), total_hand),
        };
    }

    // Check for 2pair
    pairs.push(Rank::MAX);
    pairs.push(Rank::MAX);
    let top_two_pairs: Vec<Card> = total_hand.clone().into_iter().filter(|c| -> bool {c.rank == pairs[0] || c.rank == pairs[1]}).collect();
    if top_two_pairs.len() > 0 {
        return FinalHand {
            hand_rank: if top_two_pairs.len() == 4 { TwoPair } else { Pair },
            hand: best_five(top_two_pairs, total_hand),
        };
    }

    // Return highcard
    return FinalHand {
        hand_rank: HighCard,
        hand: best_five(vec![], total_hand),
    };
}
