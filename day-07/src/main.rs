use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
enum CardType {
    Num(u8),
    T,
    J,
    Q,
    K,
    A,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
enum CardTypeJ {
    J,
    Num(u8),
    T,
    Q,
    K,
    A,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

trait CardHash {
    fn remove_j(map: &mut HashMap<Self, u8>) -> u8 where Self:Sized;
    fn increment(map: HashMap<Self, u8>, val: Self) -> HashMap<Self, u8> where Self:Sized;
}

impl CardHash for CardType {
    fn remove_j(_map: &mut HashMap<CardType, u8>) -> u8 {
        0_u8
    }

    fn increment(mut map: HashMap<CardType, u8>, val: CardType) -> HashMap<CardType, u8> {
        map.entry(val).and_modify(|freq| *freq += 1).or_insert(1);
        return map;
    }
}

impl CardHash for CardTypeJ {
    fn remove_j(map: &mut HashMap<CardTypeJ, u8>) -> u8 {
        map.remove(&CardTypeJ::J).unwrap_or(0)
    }

    fn increment(mut map: HashMap<CardTypeJ, u8>, val: CardTypeJ) -> HashMap<CardTypeJ, u8> {
        map.entry(val).and_modify(|freq| *freq += 1).or_insert(1);
        return map;
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Hand<T: PartialEq + Eq + Ord>(T, T, T, T, T, bool);

impl<T: Ord+CardHash+Copy> Hand<T> {
    fn get_type(&self) -> Result<HandType, ()> {
        let cards = [self.0, self.1, self.2, self.3, self.4];
        let mut freq = cards.into_iter().fold(
            HashMap::new(),
            T::increment
        );
        let j_count = T::remove_j(&mut freq);
        let mut heap: BinaryHeap<u8> = freq.values().map(|val| *val).collect();
        return match heap.pop().unwrap_or(0)+j_count {
            5 => Ok(HandType::FiveKind),
            4 => Ok(HandType::FourKind),
            3 => {
                if heap.pop().unwrap() == 2 {
                    Ok(HandType::FullHouse)
                } else {
                    Ok(HandType::ThreeKind)
                }
            }
            2 => {
                if heap.pop().unwrap() == 2 {
                    Ok(HandType::TwoPair)
                } else {
                    Ok(HandType::OnePair)
                }
            }
            1 => Ok(HandType::HighCard),
            _ => return Err(()),
        };
    }

    fn to_array(&self) -> [T; 5] {
        [self.0, self.1, self.2, self.3, self.4]
    }
}

impl CardType {
    fn from_str(a: &str) -> Result<CardType, ()> {
        if a.starts_with(|ch: char| ch.is_ascii_digit()) {
            return if a == "1" || a == "0" {
                Err(())
            } else {
                Ok(CardType::Num(a.parse::<u8>().unwrap()))
            };
        }
        match a {
            "A" => Ok(CardType::A),
            "K" => Ok(CardType::K),
            "Q" => Ok(CardType::Q),
            "J" => Ok(CardType::J),
            "T" => Ok(CardType::T),
            _ => return Err(()),
        }
    }

    fn from_cards(
        card1: CardType,
        card2: CardType,
        card3: CardType,
        card4: CardType,
        card5: CardType,
    ) -> Hand<CardType> {
        Hand(card1, card2, card3, card4, card5, false)
    }


    fn new_hand(string: &str) -> Result<Hand<CardType>, ()> {
        Ok(CardType::from_cards(
            CardType::from_str(&string[0..=0])?,
            CardType::from_str(&string[1..=1])?,
            CardType::from_str(&string[2..=2])?,
            CardType::from_str(&string[3..=3])?,
            CardType::from_str(&string[4..=4])?,
        ))
    }
}

impl CardTypeJ {
    fn from_str(a: &str) -> Result<CardTypeJ, ()> {
        if a.starts_with(|ch: char| ch.is_ascii_digit()) {
            return if a == "1" || a == "0" {
                Err(())
            } else {
                Ok(CardTypeJ::Num(a.parse::<u8>().unwrap()))
            };
        }
        match a {
            "A" => Ok(CardTypeJ::A),
            "K" => Ok(CardTypeJ::K),
            "Q" => Ok(CardTypeJ::Q),
            "J" => Ok(CardTypeJ::J),
            "T" => Ok(CardTypeJ::T),
            _ => return Err(()),
        }
    }

    fn from_cards(
        card1: CardTypeJ,
        card2: CardTypeJ,
        card3: CardTypeJ,
        card4: CardTypeJ,
        card5: CardTypeJ,
    ) -> Hand<CardTypeJ> {
        Hand(card1, card2, card3, card4, card5, true)
    }

    fn new_hand(string: &str) -> Result<Hand<CardTypeJ>, ()> {
        Ok(CardTypeJ::from_cards(
            CardTypeJ::from_str(&string[0..=0])?,
            CardTypeJ::from_str(&string[1..=1])?,
            CardTypeJ::from_str(&string[2..=2])?,
            CardTypeJ::from_str(&string[3..=3])?,
            CardTypeJ::from_str(&string[4..=4])?,
        ))
    }
}

impl<T: Ord+PartialOrd+CardHash+Copy> PartialOrd for Hand<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.to_array() == other.to_array() {
            return Some(Ordering::Equal);
        } else if self.get_type() != other.get_type() {
            return self
                .get_type()
                .unwrap()
                .partial_cmp(&other.get_type().unwrap());
        } else {
            let cards_self = self.to_array();
            let cards_other = other.to_array();
            for (card_self, card_other) in cards_self.into_iter().zip(cards_other.into_iter()) {
                if card_self != card_other {
                    return card_self.partial_cmp(&card_other);
                }
            }
            return Some(Ordering::Equal);
        }
    }
}

impl<T: Ord+PartialOrd+CardHash+Copy> Ord for Hand<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn part1(raw_input: String) -> u32 {
    let raw_lines: Vec<String> = raw_input.lines().map(String::from).collect();
    let mut pairs: Vec<(Hand<CardType>, u32)> = raw_lines
        .into_iter()
        .map(|line| {
            let (hand, bet) = line.split_once(' ').unwrap();
            return (
                CardType::new_hand(hand).unwrap(),
                bet.parse::<u32>().unwrap(),
            );
        })
        .collect();

    pairs.sort_by_key(|pairs| pairs.0);
    println!("{:?}", pairs);
    println!("{:?}", pairs.iter().map(|pair| pair.1).collect::<Vec<u32>>());
    return pairs
        .into_iter()
        .enumerate()
        .map(|pair| (pair.0 + 1) as u32 * pair.1 .1)
        .reduce(std::ops::Add::add)
        .unwrap();
}

fn part2(raw_input: String) -> u32 {
    let raw_lines: Vec<String> = raw_input.lines().map(String::from).collect();
    let mut pairs: Vec<(Hand<CardTypeJ>, u32)> = raw_lines
        .into_iter()
        .map(|line| {
            let (hand, bet) = line.split_once(' ').unwrap();
            return (
                CardTypeJ::new_hand(hand).unwrap(),
                bet.parse::<u32>().unwrap(),
            );
        })
        .collect();

    pairs.sort_by_key(|pairs| pairs.0);
    println!("{:?}", pairs);
    println!("{:?}", pairs.iter().map(|pair| pair.1).collect::<Vec<u32>>());
    return pairs
        .into_iter()
        .enumerate()
        .map(|pair| (pair.0 + 1) as u32 * pair.1 .1)
        .reduce(std::ops::Add::add)
        .unwrap();
}

fn main() {
    let string = fs::read_to_string("data.txt").unwrap();
    println!("{}", part1(string.clone()));
    println!("{}", part2(string));
}

mod tests {

    #[test]
    fn solved_part1() {
        let res = crate::part1(std::fs::read_to_string("test.txt").unwrap());
        println!("{}", res);
        assert_eq!(res, 6440);
    }

    #[test]
    fn solved_part2() {
        let res = crate::part2(std::fs::read_to_string("test.txt").unwrap());
        println!("{}", res);
        assert_eq!(res, 5905);
    }

}
