#[allow(unused_imports)]
use day16_ticket_translation_common::{Rule, Ticket, Notes, SAMPLE_DATA};

fn main() {
    let result = do_work(&SAMPLE_DATA);
    println!("{}", result);
}

fn do_work(data: &Notes) -> u64 {
    let mut error_rate = 0;

    for ticket in data.nearby_tickets {
        error_rate += get_invalid_fields(&data.rules, ticket);
    }

    error_rate
}

fn get_invalid_fields(rules: &[Rule], ticket: &Ticket) -> u64 {
    let mut invalid_fields = 0;

    for field in ticket.fields {
        if !is_field_valid(rules, *field) {
            invalid_fields += *field;
        }
    }

    invalid_fields
}

fn is_field_valid(rules: &[Rule], field: u64) -> bool {
    for rule in rules {
        for range in rule.ranges {
            if (field >= range.min) && (field <= range.max) {
                return true;
            }
        }
    }

    false
}
