use std::collections::{HashSet, VecDeque};

#[derive(Clone)]
struct Deck {
    owner: String,
    cards: VecDeque<u64>,
}

fn play_game(game: u64, p1: &mut Deck, p2: &mut Deck) -> String {
    let mut history: HashSet<VecDeque<u64>> = HashSet::new();

    loop {
        // Before either player deals a card, if there was a previous round in this game that
        // had exactly the same cards in the same order in the same players' decks, the game
        // instantly ends in a win for player 1. Previous rounds from other games are not
        // considered. (This prevents infinite games of Recursive Combat, which everyone agrees
        // is a bad idea.)
        if history.contains(&p1.cards) || history.contains(&p2.cards) {
            return p1.owner.clone();
        }

        history.insert(p1.cards.clone());
        history.insert(p2.cards.clone());

        // Otherwise, this round's cards must be in a new configuration; the players begin the
        // round by each drawing the top card of their deck as normal.
        let p1_c = p1.cards.pop_front().unwrap();
        let p2_c = p2.cards.pop_front().unwrap();

        // If both players have at least as many cards remaining in their deck as the value of
        // the card they just drew, the winner of the round is determined by playing a new game
        // of Recursive Combat.
        let winner = if p1.cards.len() as u64 >= p1_c && p2.cards.len() as u64 >= p2_c {
            // To play a sub-game of Recursive Combat, each player creates a new deck by making
            // a copy of the next cards in their deck (the quantity of cards copied is equal to the
            // number on the card they drew to trigger the sub-game).
            let mut p1_copy = p1.clone();
            p1_copy.cards = p1.cards.iter().take(p1_c as usize).map(|c| *c).collect();
            let mut p2_copy = p2.clone();
            p2_copy.cards = p2.cards.iter().take(p2_c as usize).map(|c| *c).collect();

            play_game(game + 1, &mut p1_copy, &mut p2_copy)
        // Otherwise, at least one player must not have enough cards left in their deck to recurse; the
        // winner of the round is the player with the higher-value card.
        } else if p1_c > p2_c {
            p1.owner.clone()
        } else {
            p2.owner.clone()
        };

        if winner == p1.owner {
            p1.cards.push_back(p1_c);
            p1.cards.push_back(p2_c);
        } else {
            p2.cards.push_back(p2_c);
            p2.cards.push_back(p1_c);
        };

        if p1.cards.is_empty() {
            return p2.owner.clone();
        }

        if p2.cards.is_empty() {
            return p1.owner.clone();
        }
    }
}

fn score(d: &Deck) -> u64 {
    d.cards
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) as u64 * c)
        .sum()
}

fn get_player<'a>(owner: &str, p1: &'a Deck, p2: &'a Deck) -> &'a Deck {
    if owner == p1.owner {
        p1
    } else {
        p2
    }
}

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = Deck> + 'a {
    input.split("\n\n").map(|d| {
        let mut lines = d.lines();
        let owner = lines.next().unwrap().trim_end_matches(':').to_string();
        let cards = lines.map(|l| l.parse().unwrap()).collect();

        Deck { owner, cards }
    })
}

fn part_one(input: &str) -> String {
    let mut decks = parse_input(input);
    let mut p1 = decks.next().unwrap();
    let mut p2 = decks.next().unwrap();

    loop {
        if p1.cards.is_empty() || p2.cards.is_empty() {
            break;
        }

        let p1_c = p1.cards.pop_front().unwrap();
        let p2_c = p2.cards.pop_front().unwrap();

        if p1_c > p2_c {
            p1.cards.push_back(p1_c);
            p1.cards.push_back(p2_c);
        } else {
            p2.cards.push_back(p2_c);
            p2.cards.push_back(p1_c);
        };
    }


    let winner = if p1.cards.is_empty() { p2 } else { p1 };
    score(&winner).to_string()
}

fn part_two(input: &str) -> String {
    let mut decks = parse_input(input);
    let mut p1 = decks.next().unwrap();
    let mut p2 = decks.next().unwrap();

    let winner = play_game(1, &mut p1, &mut p2);

    score(get_player(&winner, &p1, &p2)).to_string()
}

fn main() {
    let input = include_str!("../../../../input/2020/day22.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 22, 1, 1);
    test_example!(example_two_1, part_two, 22, 2, 1);
}
