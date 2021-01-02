use std::collections::HashMap;

use rayon::prelude::*;

#[allow(unused_imports)]
use day19_monster_messages_common::{RuleDescription, Rule, TARGET_RULE, SAMPLE_RULES, SAMPLE_MESSAGES, REAL_RULES, REAL_MESSAGES};

fn main() {
    let result = do_work(&REAL_RULES, &REAL_MESSAGES);
    println!("{}", result);
}

fn do_work(rule_list: &[Rule], messages: &[&str]) -> usize {
    let mut rules = HashMap::new();
    for rule in rule_list.iter() {
        rules.insert(rule.id, &rule.description);
    }

    let matching_count = messages.par_iter().map(|message| {
        if does_message_match_rules(message, &rules) {
            return 1;
        } else {
            return 0;
        }
    }).sum();

    matching_count
}

fn does_message_match_rules(message: &str, rules: &HashMap::<u64, &RuleDescription>) -> bool {
    let (matched, remaining) = does_fragment_match_rule(message, rules, TARGET_RULE);

    if matched && remaining.len() == 0 {
        return true;
    }

    false
}

fn does_fragment_match_rule<'a>(
    fragment: &'a str,
    rules: &HashMap::<u64, &RuleDescription>,
    rule_id: u64) -> (bool, &'a str) {
    if fragment.len() == 0 {
        return (false, fragment);
    }

    let rule = rules.get(&rule_id).unwrap();

    match rule {
        RuleDescription::Literal(target_char) => {
            let first_char = fragment.chars().nth(0).unwrap();
            if first_char == *target_char {
                return (true, &fragment[1..]);
            } else {
                return (false, fragment);
            }
        },
        RuleDescription::Sequence(sequence) => {
            return does_fragment_match_rule_sequence(fragment, rules, sequence);
        },
        RuleDescription::Or(first_seq, second_seq) => {
            let (matched, remaining) = does_fragment_match_rule_sequence(fragment, rules, first_seq);
            if matched {
                return (matched, remaining);
            } else {
                return does_fragment_match_rule_sequence(fragment, rules, second_seq);
            }
        }
    }
}

fn does_fragment_match_rule_sequence<'a>(
    fragment: &'a str,
    rules: &HashMap::<u64, &RuleDescription>,
    rule_sequence: &[u64]
    ) -> (bool, &'a str) {
    let mut matched = true;
    let mut remaining = fragment;

    for next_rule_id in rule_sequence.iter() {
        let result = does_fragment_match_rule(remaining, rules, *next_rule_id);
        matched = result.0;
        remaining = result.1;

        if !matched {
            return (matched, remaining);
        }
    }

    (matched, remaining)
}