use std::collections::HashMap;
use std::path::Path;
use crate::utils::read_lines;

#[derive(Debug)]
struct Hand {
    strength: String,
    bid: u64,
    card_map: HashMap<char, u64>,
}

fn format_data(file: &Path, has_joker: bool) -> (Vec<Hand>, HashMap<char, u64>) {
    let mut camel_cards: Vec<Hand> = vec![];
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            let camel_card = strs.split(' ').filter(|s| !s.is_empty()).collect::<Vec<&str>>();
            let mut camel_card_map: HashMap<char, u64> = HashMap::new();
            let mut joker_card_map: HashMap<char, u64> = HashMap::new();

            camel_card[0].chars().for_each(|c| {
                if camel_card_map.get(&c).is_some() {
                    camel_card_map.insert(c, camel_card_map.get(&c).unwrap() + 1);
                } else {
                    camel_card_map.insert(c, 1);
                }
            });

            if has_joker {
                joker_card_map = camel_card_map.clone();
                if camel_card_map.contains_key(&'J') {
                    joker_card_map.remove(&'J');
                    let mut max_key = ' ';
                    let mut max_value = u64::MIN;
                    joker_card_map.iter().for_each(|(k, v)| {
                        if *v > max_value {
                            max_value = *v;
                            max_key = *k;
                        }
                    });
                    joker_card_map.insert(max_key, camel_card_map.get(&'J').unwrap() + max_value);
                }
                camel_card_map  = joker_card_map;
            }
            camel_cards.push(Hand {
                strength: camel_card[0].parse().unwrap(),
                card_map: camel_card_map,
                bid: camel_card[1].parse::<u64>().unwrap()
            });
        });

    let mut j_val = 11;
    if has_joker {
        j_val = 1;
    }
    let char_rank = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', j_val),
    ]);

    (camel_cards, char_rank)
}

// 1, 2, 2,  3, 3, 4, 5
fn total_winnings(hands: Vec<Hand>, char_rank: HashMap<char, u64>) -> u64 {
    let mut sort_hand_group = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
    for hand in hands {
        if hand.card_map.len() == 5 {
            sort_hand_group[0].push(hand);
        } else if hand.card_map.len() == 4 {
            sort_hand_group[1].push(hand);
        } else if hand.card_map.len() == 3 {
            if hand.card_map.values().product::<u64>() == 4 {
                sort_hand_group[2].push(hand);
            } else {
                sort_hand_group[3].push(hand);
            }
        } else if hand.card_map.len() == 2 {
            if hand.card_map.values().product::<u64>() == 6 {
                sort_hand_group[4].push(hand);
            } else {
                sort_hand_group[5].push(hand);
            }
        } else {
            sort_hand_group[6].push(hand);
        }
    }

    sort_hand_group.iter_mut().for_each(|h| {
        h.sort_by(|a, b| {
            for (idx, c) in a.strength.chars().enumerate() {
                let bc = b.strength.chars().nth(idx).unwrap();
                if c != bc {
                    return char_rank.get(&c).unwrap().cmp(char_rank.get(&bc).unwrap());
                }
            }
            std::cmp::Ordering::Equal
        });
    });

    let sorted_hands = sort_hand_group.into_iter().flatten().collect::<Vec<Hand>>();
    sorted_hands.iter().enumerate().map(|(idx, h)| {
        h.bid * (idx as u64 + 1)
    })
        .sum()
}

#[cfg(test)]
mod day7_tests {
    use super::*;

    #[test]
    fn day7_1_test() {
        let (hands,char_rank) = format_data(Path::new("src/day7/day7_input_test.txt"), false);
        let result = total_winnings(hands, char_rank);
        assert_eq!(result, 6440);
    }

    #[test]
    fn day7_1_answer() {
        let (hands, char_rank) = format_data(Path::new("src/day7/day7_input.txt"), false);
        let result = total_winnings(hands, char_rank);
        assert_eq!(result, 246795406);
    }

    #[test]
    fn day7_2_test() {
        let (hands, char_rank) = format_data(Path::new("src/day7/day7_input_test.txt"), true);
        let result = total_winnings(hands, char_rank);
        assert_eq!(result, 5905);
    }

    #[test]
    fn day7_2_answer() {
        let (hands, char_rank) = format_data(Path::new("src/day7/day7_input.txt"), true);
        let result = total_winnings(hands, char_rank);
        assert_eq!(result, 249356515);
    }
}
