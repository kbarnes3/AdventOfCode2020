use std::collections::HashMap;
use std::vec::Vec;

use rayon::prelude::*;

#[allow(unused_imports)]
use day19_monster_messages_common::{
    RuleDescription, Rule, TARGET_RULE, REPLACEMENT_RULES, 
    SAMPLE_RULES, SAMPLE_MESSAGES, 
    SAMPLE_RULES_2, SAMPLE_MESSAGES_2,
    REAL_RULES, REAL_MESSAGES};

fn main() {
    let result = do_work(&REAL_RULES, &REAL_MESSAGES);
    println!("{}", result);
}

fn do_work(rule_list: &[Rule], messages: &[&str]) -> usize {
    let mut rules = HashMap::new();
    for rule in rule_list.iter() {
        rules.insert(rule.id, &rule.description);
    }

    for rule in REPLACEMENT_RULES.iter() {
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
    let matched_fragments = get_fragments_matching_rule(message, rules, TARGET_RULE);

    for fragment in matched_fragments.iter() {
        if fragment.len() == 0 {
            return true;
        }
    }

    false
}

fn get_fragments_matching_rule<'a>(
    fragment: &'a str,
    rules: &HashMap::<u64, &RuleDescription>,
    rule_id: u64) -> Vec::<&'a str> {
    let mut matching_fragments = Vec::new();

    if fragment.len() == 0 {
        return matching_fragments;
    }

    let rule = rules.get(&rule_id).unwrap();

    match rule {
        RuleDescription::Literal(target_char) => {
            let first_char = fragment.chars().nth(0).unwrap();
            if first_char == *target_char {
                matching_fragments.push(&fragment[1..]);
            }
        },
        RuleDescription::Sequence(sequence) => {
            let mut result = get_fragments_matching_rule_sequence(fragment, rules, sequence);
            matching_fragments.append(&mut result);
        },
        RuleDescription::Or(first_seq, second_seq) => {
            let mut result = get_fragments_matching_rule_sequence(fragment, rules, first_seq);
            matching_fragments.append(&mut result);

            let mut result = get_fragments_matching_rule_sequence(fragment, rules, second_seq);
            matching_fragments.append(&mut result);
        }
    }

    matching_fragments
}

fn get_fragments_matching_rule_sequence<'a>(
    fragment: &'a str,
    rules: &HashMap::<u64, &RuleDescription>,
    rule_sequence: &[u64]
    ) -> Vec::<&'a str> {
    let mut matching_fragments = Vec::with_capacity(1);
    matching_fragments.push(fragment);

    for next_rule_id in rule_sequence.iter() {
        let mut next_rule_fragments = Vec::new();

        for starting_fragment in matching_fragments.iter() {
            let mut result = get_fragments_matching_rule(starting_fragment, rules, *next_rule_id);
            next_rule_fragments.append(&mut result);
        }

        matching_fragments = next_rule_fragments;

    }
    
    matching_fragments
}