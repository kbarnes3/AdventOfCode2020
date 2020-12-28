use std::collections::HashSet;
use std::vec::Vec;
#[allow(unused_imports)]
use day16_ticket_translation_common::{Rule, Ticket, Notes, SAMPLE_DATA_2, REAL_DATA};

fn main() {
    let result = do_work(&SAMPLE_DATA_2);
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

        }
    }

    0
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
