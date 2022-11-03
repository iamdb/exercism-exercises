use std::collections::HashSet;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let table = Table::new(hands.to_vec());

    if table.duplicate_hands() || hands.len() == 1 {
        hands.to_vec()
    } else {
        table
            .best_hands()
            .iter()
            .map(|h| h.hand)
            .collect::<Vec<&'a str>>()
    }
}

struct Table<'a> {
    hands: Vec<&'a str>,
}

impl<'a> Table<'a> {
    fn new(hands: Vec<&'a str>) -> Table<'a> {
        Table { hands }
    }

    fn duplicate_hands(&self) -> bool {
        let mut hands = self.hands.to_vec();
        hands.dedup();

        hands.len() == 1
    }

    fn hands(&self) -> Vec<Hand<'a>> {
        self.hands
            .iter()
            .map(|h| Hand::new(h))
            .collect::<Vec<Hand<'_>>>()
    }

    fn best_hands(&self) -> Vec<Hand<'a>> {
        let mut hands = self.hands();

        hands.iter_mut().for_each(|h| h.calculate());

        hands.sort_by(|a, b| {
            b.kind
                .partial_cmp(&a.kind)
                .unwrap_or(std::cmp::Ordering::Greater)
        });

        let first = hands.remove(0);
        let second = hands.remove(0);

        if first.kind == second.kind {
            vec![first, second]
        } else {
            vec![first]
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand<'a> {
    hand: &'a str,
    cards: Vec<Card>,
    kind: Kind,
}

impl<'a> Hand<'a> {
    fn new(hand: &'a str) -> Hand {
        let mut cards = hand
            .split_whitespace()
            .map(Card::from_str)
            .collect::<Vec<Card>>();
        cards.sort();

        Hand {
            hand,
            kind: Kind::HighCard(
                cards.get(0).unwrap().get_rank(),
                cards.get(1).unwrap().get_rank(),
                cards.get(2).unwrap().get_rank(),
                cards.get(3).unwrap().get_rank(),
                cards.get(4).unwrap().get_rank(),
            ),
            cards,
        }
    }

    fn calculate(&mut self) {
        if let (Some(flush), Some(straight)) = (self.flush(), self.straight()) {
            if let Some(ranks) = straight.get_ranks() {
                self.kind = Kind::StraightFlush(
                    flush.get_suit().unwrap().clone(),
                    ranks.get(0).unwrap().clone(),
                );
            }
        } else if let Some(quad) = self.quad() {
            self.kind = quad;
        } else if let Some(flush) = self.flush() {
            self.kind = flush;
        } else if let Some(straight) = self.straight() {
            self.kind = straight;
        } else if let (Some(pairs), Some(triplet)) = (self.pairs(), self.triplets()) {
            if let (Some(mut pairs_rank), Some(mut triplet_rank)) =
                (pairs.get_ranks(), triplet.get_ranks())
            {
                self.kind = Kind::FullHouse(
                    triplet_rank.remove(0),
                    pairs_rank.remove(0),
                    triplet.get_kicker().unwrap().clone(),
                );
            }
        } else if let Some(triplet) = self.triplets() {
            self.kind = triplet;
        } else if let Some(pair) = self.pairs() {
            self.kind = pair;
        }
    }

    fn ranks(&self) -> Vec<Rank> {
        self.cards
            .iter()
            .map(|c| c.get_rank())
            .collect::<Vec<Rank>>()
    }

    fn suits(&self) -> Vec<Suit> {
        self.cards
            .iter()
            .map(|c| c.get_suit())
            .collect::<Vec<Suit>>()
    }

    fn flush(&mut self) -> Option<Kind> {
        let mut suits = self.suits();
        suits.dedup();

        if suits.len() == 1 {
            let high_card = self.ranks().into_iter().max().unwrap();
            Some(Kind::Flush(high_card, suits.remove(0)))
        } else {
            None
        }
    }

    fn pairs(&mut self) -> Option<Kind> {
        let mut pairs = HashSet::new();
        let mut remaining = HashSet::new();

        let ranks = self.ranks();

        for rank in ranks.iter() {
            if ranks.iter().filter(|r| r == &rank).count() == 2 {
                pairs.insert(rank.clone());
            } else {
                remaining.insert(rank.clone());
            }
        }

        if pairs.len() == 1 {
            let mut pairs = pairs.drain().collect::<Vec<Rank>>();
            let high_card = remaining.iter().max().unwrap();

            Some(Kind::Pair(pairs.remove(0), high_card.clone()))
        } else if pairs.len() == 2 {
            let mut pairs = pairs.drain().collect::<Vec<Rank>>();
            pairs.sort();
            pairs.reverse();

            let high_card = remaining.iter().max().unwrap();

            Some(Kind::TwoPair(
                pairs.remove(0),
                pairs.remove(0),
                high_card.clone(),
            ))
        } else {
            None
        }
    }

    fn triplets(&mut self) -> Option<Kind> {
        let ranks = self.ranks();

        for rank in ranks.iter() {
            let all = ranks.iter().filter(|r| r == &rank).count();

            if all == 3 {
                let max = ranks.iter().filter(|r| r != &rank).max().unwrap();

                return Some(Kind::ThreeOfAKind(rank.clone(), max.clone()));
            }
        }

        None
    }

    fn quad(&self) -> Option<Kind> {
        let mut quad = HashSet::new();
        let mut remaining = HashSet::new();

        let ranks = self.ranks();

        for rank in ranks.iter() {
            if quad.is_empty() && ranks.iter().filter(|r| r == &rank).count() == 4 {
                quad.insert(rank.clone());
            } else {
                remaining.insert(rank.clone());
            }
        }

        if quad.len() == 1 {
            Some(Kind::FourOfAKind(
                quad.drain().collect::<Vec<Rank>>().remove(0),
                remaining.iter().max().unwrap().clone(),
            ))
        } else {
            None
        }
    }

    fn straight(&mut self) -> Option<Kind> {
        if !self.consecutive_ranks() {
            None
        } else {
            let mut ranks = self.ranks();
            ranks.reverse();

            if self.ace_low() {
                Some(Kind::Straight(Rank::N5, Rank::N4, Rank::N3))
            } else {
                ranks.get(0..3).map(|ranks| {
                    Kind::Straight(ranks[0].clone(), ranks[1].clone(), ranks[2].clone())
                })
            }
        }
    }

    fn ace_low(&self) -> bool {
        let mut ranks = self.ranks();
        ranks.sort();

        self.consecutive_ranks()
            && ranks.first() == Some(&Rank::N2)
            && ranks.last() == Some(&Rank::A)
    }

    fn consecutive_ranks(&self) -> bool {
        let mut ranks = self.ranks();
        ranks.sort();

        for window in ranks.windows(2) {
            let rank1 = &window[0];
            let rank1_val = rank1.clone() as i32;
            let rank2 = &window[1];
            let mut rank2_val = rank2.clone() as i32;

            // Low Ace Straight
            if rank2 == &Rank::A && rank1 == &Rank::N5 {
                rank2_val = 6;
            }

            if rank2_val != rank1_val + 1 {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    /// (complete hand, sorted by card in descending rank)
    HighCard(Rank, Rank, Rank, Rank, Rank),
    /// (pair_rank, kicker_rank)
    Pair(Rank, Rank),
    /// (high_pair_rank, low_pair_rank, kicker_rank)
    TwoPair(Rank, Rank, Rank),
    /// (triplet_rank, kicker_rank)
    ThreeOfAKind(Rank, Rank),
    /// (top 3 cards)
    Straight(Rank, Rank, Rank),
    /// (suit, high_card_rank)
    Flush(Rank, Suit),
    /// (triplet, pair, kicker)
    FullHouse(Rank, Rank, Rank),
    /// (quad_rank, kicker_rank)
    FourOfAKind(Rank, Rank),
    /// (suit, kicker_rank)
    StraightFlush(Suit, Rank),
}

impl Kind {
    fn get_ranks(&self) -> Option<Vec<Rank>> {
        match self {
            Kind::HighCard(r1, r2, r3, r4, r5) => Some(vec![
                r1.clone(),
                r2.clone(),
                r3.clone(),
                r4.clone(),
                r5.clone(),
            ]),
            Kind::Pair(r, _) => Some(vec![r.clone()]),
            Kind::TwoPair(r1, r2, _) => Some(vec![r1.clone(), r2.clone()]),
            Kind::ThreeOfAKind(r, _) => Some(vec![r.clone()]),
            Kind::Straight(r, _, _) => Some(vec![r.clone()]),
            Kind::Flush(_, _) => None,
            Kind::FullHouse(r, _, _) => Some(vec![r.clone()]),
            Kind::FourOfAKind(r, _) => Some(vec![r.clone()]),
            Kind::StraightFlush(_, r) => Some(vec![r.clone()]),
        }
    }

    fn get_kicker(&self) -> Option<&Rank> {
        match self {
            Kind::HighCard(_, _, _, _, _) => None,
            Kind::Pair(_, r) => Some(r),
            Kind::TwoPair(_, _, r) => Some(r),
            Kind::ThreeOfAKind(_, r) => Some(r),
            Kind::Straight(_, _, r) => Some(r),
            Kind::Flush(_, _) => None,
            Kind::FullHouse(_, _, r) => Some(r),
            Kind::FourOfAKind(_, r) => Some(r),
            Kind::StraightFlush(_, r) => Some(r),
        }
    }

    fn get_suit(&self) -> Option<&Suit> {
        match self {
            Kind::HighCard(_, _, _, _, _) => None,
            Kind::Pair(_, _) => None,
            Kind::TwoPair(_, _, _) => None,
            Kind::ThreeOfAKind(_, _) => None,
            Kind::Straight(_, _, _) => None,
            Kind::Flush(_, s) => Some(s),
            Kind::FullHouse(_, _, _) => None,
            Kind::FourOfAKind(_, _) => None,
            Kind::StraightFlush(s, _) => Some(s),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Card((Rank, Suit));

impl Card {
    fn new(rank: Rank, suit: Suit) -> Card {
        Card((rank, suit))
    }

    fn from_str(s: &str) -> Card {
        let suit: Suit = s.chars().last().unwrap().into();
        let rank = &s[0..s.len() - 1];

        Card::new(rank.into(), suit)
    }

    fn get_rank(&self) -> Rank {
        self.0 .0.clone()
    }

    fn get_suit(&self) -> Suit {
        self.0 .1.clone()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
enum Rank {
    N1 = 1,
    N2 = 2,
    N3 = 3,
    N4 = 4,
    N5 = 5,
    N6 = 6,
    N7 = 7,
    N8 = 8,
    N9 = 9,
    N10 = 10,
    J = 11,
    Q = 12,
    K = 13,
    A = 14,
    Unknown = 0,
}

impl From<&str> for Rank {
    fn from(s: &str) -> Self {
        match s {
            "1" => Self::N1,
            "2" => Self::N2,
            "3" => Self::N3,
            "4" => Self::N4,
            "5" => Self::N5,
            "6" => Self::N6,
            "7" => Self::N7,
            "8" => Self::N8,
            "9" => Self::N9,
            "10" => Self::N10,
            "J" => Self::J,
            "Q" => Self::Q,
            "K" => Self::K,
            "A" => Self::A,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Suit {
    Club = 3,
    Diamond = 1,
    Heart = 2,
    Spade = 4,
    Unknown = 0,
}

impl From<char> for Suit {
    fn from(c: char) -> Self {
        match c {
            'C' => Self::Club,
            'D' => Self::Diamond,
            'H' => Self::Heart,
            'S' => Self::Spade,
            _ => Self::Unknown,
        }
    }
}
