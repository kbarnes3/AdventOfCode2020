use std::collections::HashSet;
use std::vec::Vec;
#[allow(unused_imports)]
use day16_ticket_translation_common::{Rule, Ticket, Notes, SAMPLE_DATA_2, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work(data: &Notes) -> u64 {
    let mut possible_rules = Vec::with_capacity(data.my_ticket.fields.len());

    for _ in 0..data.my_ticket.fields.len() {
        let mut candidate_rules = HashSet::new();
        for rule in data.rules.iter() {
            candidate_rules.insert(rule);
        }

        possible_rules.push(candidate_rules);
    }

    for ticket in data.nearby_tickets {
        if !has_invalid_fields(&data.rules, ticket) {
            for (i, field) in ticket.fields.iter().enumerate() {
                let candidate_rules = possible_rules.get_mut(i).unwrap();
                let mut rules_to_remove = Vec::new();

                for rule in candidate_rules.iter() {
                    if !does_field_match_rule(rule, *field) {
                        rules_to_remove.push(*rule);
                    }
                }

                for rule in rules_to_remove.iter() {
                    candidate_rules.remove(rule);
                }
            }
        }
    }

    let mut progress_made = true;
    let mut all_matched = false;
    let mut known_rules = HashSet::with_capacity(data.rules.len());

    while progress_made && !all_matched {
        progress_made = false;
        all_matched = true;

        for candidate_rules in possible_rules.iter_mut() {
            if candidate_rules.len() != 1 {
                for known_rule in known_rules.iter() {
                    progress_made |= candidate_rules.remove(known_rule);
                }
            }

            if candidate_rules.len() == 1 {
                for rule in candidate_rules.iter() {
                    known_rules.insert(*rule);
                }
            } else {
                all_matched = false;
            }
        }

    }

    if !all_matched {
        println!("Warning: Not all fields are matched to rules");
    }

    let mut departure_product = 1;

    for (i, candidate_rules) in possible_rules.iter().enumerate() {
        if candidate_rules.len() != 1 {
            println!("Warning: Field {} has {} candidate rules", i, candidate_rules.len());
        }

        for rule in candidate_rules.iter() {
            // println!("{}: {}", i, rule.name);
            if rule.name.starts_with("departure") {
                departure_product *= data.my_ticket.fields[i];
            }
        }
    }

    departure_product
}

fn has_invalid_fields(rules: &[Rule], ticket: &Ticket) -> bool {

    for field in ticket.fields {
        if !is_field_valid(rules, *field) {
            return true
        }
    }

    false
}

fn is_field_valid(rules: &[Rule], field: u64) -> bool {
    for rule in rules {
        if does_field_match_rule(rule, field) {
            return true;
        }
    }

    false
}

fn does_field_match_rule(rule: &Rule, field: u64) -> bool {
    for range in rule.ranges {
        if (field >= range.min) && (field <= range.max) {
            return true;
        }
    }

    false
}
