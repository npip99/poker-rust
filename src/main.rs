mod hand;
mod deck;

use deck::Card;
use deck::Deck;
use hand::best_five_of_seven;

fn main() {
    let mut d = Deck::new();
    for i in 0..20 {
        d.shuffle();
        let mut hand = vec![];
        for i in 0..7 {
            hand.push(d.draw_card());
        }
        print!("Board: ");
        for c in &hand {
            print!("{} ", c);
        }
        let final_hand = best_five_of_seven(hand);
        print!("\nBest Hand: {:?} ", final_hand.hand_rank);
        for c in final_hand.hand {
            print!("{} ", c);
        }
        println!();
    }
}
