#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace
}

impl Card {
    fn from_char(c: char) -> Option<Card> {
        match c {
            'J' => Some(Card::Joker),
            '2' => Some(Card::Two),
            '3' => Some(Card::Three),
            '4' => Some(Card::Four),
            '5' => Some(Card::Five),
            '6' => Some(Card::Six),
            '7' => Some(Card::Seven),
            '8' => Some(Card::Eight),
            '9' => Some(Card::Nine),
            'T' => Some(Card::Ten),
            'Q' => Some(Card::Queen),
            'K' => Some(Card::King),
            'A' => Some(Card::Ace),
            _ => None
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

impl HandType {
    fn from_cards(cards: [Card; 5]) -> HandType {
        let (freq, jokers) = frequencies(cards);
        let (fst, snd) = (freq[0], freq[1]);
        match (fst + jokers, snd) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            _ => HandType::HighCard
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    bet: u32
}

fn frequencies(cards: [Card; 5]) -> ([u32; 13], u32) {
    let mut card_frequencies = [0; 13];
    let mut jokers = 0;
    for &card in cards.iter() {
        match card {
            Card::Joker => {jokers += 1;},
            _ => {card_frequencies[card as usize] += 1;}
        };
    }
    card_frequencies.sort_unstable_by(|a, b| b.cmp(a));  // descending order
    (card_frequencies, jokers)
}

fn parse_hand(line: &str) -> Hand {
    let (cards, bet) = line.split_once(" ").expect("invalid line format");
    let mut parsed_cards = [Card::Ace; 5];
    for (i, c) in cards.chars().enumerate() {
        parsed_cards[i] = Card::from_char(c).expect("invalid card");
    }
    let hand_type = HandType::from_cards(parsed_cards);
    let bet = bet.parse::<u32>().expect("invalid bet");
    Hand {
        cards: parsed_cards,
        hand_type,
        bet
    }
}

fn calculate_total(hands: &[Hand]) -> u32 {
    hands.iter().enumerate().map(|(position, hand)| (position + 1) as u32 * hand.bet).sum()
}

pub fn part_one(input: &str) -> u32 {
    0
}

pub fn part_two(input: &str) -> u32 {
    let mut hands = input.lines().map(parse_hand).collect::<Vec<_>>();
    hands.sort();
    calculate_total(&hands)
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("inputs", 7);
        assert_eq!(part_one(&input), 248422077);

    }

    #[test]
    fn test_part_two() {
        let input = read_file("inputs", 7);
        assert_eq!(part_two(&input), 249817836);
    }

    #[test]
    fn card_ordering() {
        assert!(Card::Two < Card::Three);
        assert!(Card::Three < Card::Four);
    }

    #[test]
    fn hand_type_ordering() {
        assert!(HandType::HighCard < HandType::OnePair);
        assert!(HandType::OnePair < HandType::TwoPair);
        // Add more assertions for the rest of the hand types
    }

    #[test]
    fn hand_ordering() {
        let hand1 = Hand {
            cards: [Card::Ace, Card::Ace, Card::Two, Card::Three, Card::Four],
            hand_type: HandType::OnePair,
            bet: 1
        };
        let hand2 = Hand {
            cards: [Card::Two, Card::Two, Card::Two, Card::Two, Card::Two],
            hand_type: HandType::FiveOfAKind,
            bet: 2
        };
        assert!(hand1 < hand2);
    }
}