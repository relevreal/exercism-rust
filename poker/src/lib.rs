use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands: Vec<_> = hands
        .iter()
        .map(|&s| (PokerHand::from_str(s).unwrap(), s))
        .collect();

    hands.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(Ordering::Equal));
    let first_hand = hands.first().unwrap();
    let result: Vec<&str> = hands
        .iter()
        .filter(|h| h.0 == first_hand.0)
        .map(|h| h.1)
        .collect();
    result
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum Category {
    HighCard([Rank; 5]),
    OnePair(Rank, [Rank; 3]),
    TwoPair(Rank, Rank, Rank),
    ThreeOfKind(Rank, [Rank; 2]),
    Straight(Rank),
    Flush(Rank, [Rank; 4]),
    FullHouse(Rank, Rank),
    FourOfKind(Rank, Rank),
    StraightFlush(Rank),
}

#[derive(Debug)]
struct ParsePokerHandError;

#[derive(Debug, Clone)]
struct PokerHand {
    cards: Vec<Card>,
}

impl PartialEq<Self> for PokerHand {
    fn eq(&self, other: &Self) -> bool {
        let category = self.category();
        let other_category = other.category();
        category == other_category
    }
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let category = self.category();
        let other_category = other.category();
        let ordering = category.cmp(&other_category);
        Some(ordering)
    }
}

impl PokerHand {
    pub fn category(&self) -> Category {
        let is_one_suit = self.cards.iter().all(|c| c.suit == self.cards[0].suit);
        let desc_rank_counts = self.get_rank_counts();
        let mut sorted_ranks: Vec<_> = self.cards.iter().map(|c| c.rank).collect();
        sorted_ranks.sort_by_key(|r| Reverse(*r));
        let is_ascending = self.is_ascending(&sorted_ranks);
        match (is_one_suit, is_ascending, desc_rank_counts) {
            (true, true, [(1, r1), _, _, _, _]) => Category::StraightFlush(r1),
            (_, _, [(4, r1), (1, r2), _, _, _]) => Category::FourOfKind(r1, r2),
            (_, _, [(3, r1), (2, r2), _, _, _]) => Category::FullHouse(r1, r2),
            (true, _, [(1, r1), (1, r2), (1, r3), (1, r4), (1, r5)]) => {
                let kickers = sort_kickers([r2, r3, r4, r5]);
                if r1 == Rank::A && kickers == [Rank::Five, Rank::Four, Rank::Three, Rank::Two] {
                    Category::StraightFlush(Rank::Five)
                } else {
                    Category::Flush(r1, kickers)
                }
            }
            (_, true, [(1, r1), _, _, _, _]) => Category::Straight(r1),
            (_, _, [(3, r1), (1, k1), (1, k2), _, _]) => {
                Category::ThreeOfKind(r1, sort_kickers([k1, k2]))
            }
            (_, _, [(2, r1), (2, r2), (1, kicker), _, _]) => Category::TwoPair(r1, r2, kicker),
            (_, _, [(2, r1), (1, r2), (1, r3), (1, r4), _]) => {
                Category::OnePair(r1, sort_kickers([r2, r3, r4]))
            }
            (_, _, [(1, r1), (1, r2), (1, r3), (1, r4), (1, r5)]) => {
                let kickers = sort_kickers([r1, r2, r3, r4, r5]);
                if kickers == [Rank::A, Rank::Five, Rank::Four, Rank::Three, Rank::Two] {
                    Category::Straight(Rank::Five)
                } else {
                    Category::HighCard(kickers)
                }
            }
            _ => panic!("???"),
        }
    }

    fn is_ascending(&self, sorted_ranks: &[Rank]) -> bool {
        sorted_ranks.windows(2).rev().all(|w| w[1].next() == w[0])
    }

    fn get_rank_counts(&self) -> [(usize, Rank); 5] {
        let mut counter: HashMap<Rank, usize> = HashMap::new();
        for card in self.cards.iter() {
            *counter.entry(card.rank).or_insert(0) += 1;
        }
        let mut v = Vec::from_iter(counter.into_iter());
        v.sort_by_key(|(k, v)| Reverse((*v, *k)));
        let mut arr = [(0_usize, Rank::Empty); 5];
        for (i, (rank, count)) in v.into_iter().enumerate() {
            arr[i] = (count, rank);
        }
        arr
    }
}

fn sort_kickers<const N: usize>(mut kickers: [Rank; N]) -> [Rank; N] {
    kickers.sort_by(|a, b| b.cmp(&a));
    kickers
}

impl FromStr for PokerHand {
    type Err = ParsePokerHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards_str: Vec<&str> = s.split(" ").collect();
        let cards = vec![
            Card::from_str(cards_str[0])?,
            Card::from_str(cards_str[1])?,
            Card::from_str(cards_str[2])?,
            Card::from_str(cards_str[3])?,
            Card::from_str(cards_str[4])?,
        ];

        Ok(Self { cards })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Card {
    suit: Suit,
    rank: Rank,
}

impl FromStr for Card {
    type Err = ParsePokerHandError;

    fn from_str(card_str: &str) -> Result<Self, Self::Err> {
        let card_len = card_str.len();
        if card_len != 2 && card_len != 3 {
            return Err(ParsePokerHandError);
        }
        let suit = match card_str.chars().last().unwrap() {
            'C' => Suit::Club,
            'D' => Suit::Diamond,
            'H' => Suit::Heart,
            'S' => Suit::Spade,
            _ => return Err(ParsePokerHandError),
        };

        let rank = match &card_str[..(card_len - 1)] {
            "2" => Rank::Two,
            "3" => Rank::Three,
            "4" => Rank::Four,
            "5" => Rank::Five,
            "6" => Rank::Six,
            "7" => Rank::Seven,
            "8" => Rank::Eight,
            "9" => Rank::Nine,
            "10" => Rank::Ten,
            "J" => Rank::J,
            "Q" => Rank::Q,
            "K" => Rank::K,
            "A" => Rank::A,
            _ => return Err(ParsePokerHandError),
        };

        Ok(Self { suit, rank })
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
enum Rank {
    Empty,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

impl Rank {
    fn next(&self) -> Self {
        match self {
            Self::Empty => Self::Empty,
            Self::Two => Self::Three,
            Self::Three => Self::Four,
            Self::Four => Self::Five,
            Self::Five => Self::Six,
            Self::Six => Self::Seven,
            Self::Seven => Self::Eight,
            Self::Eight => Self::Nine,
            Self::Nine => Self::Ten,
            Self::Ten => Self::J,
            Self::J => Self::Q,
            Self::Q => Self::K,
            Self::K => Self::A,
            Self::A => Self::Two,
        }
    }
}
