use std::vec::Vec;
#[allow(unused_imports)]
use day11_seating_system_common::{Position, SAMPLE_DATA, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work<Grid: AsRef<[Row]>, Row: AsRef<[Position]>>(data: Grid) -> usize {
    let mut old_positions = Vec::with_capacity(data.as_ref().len());
    let mut new_positions = Vec::with_capacity(data.as_ref().len());

    fill_initial_positions(data, &mut new_positions);

    while !are_equal(&old_positions, &new_positions) {
        copy_positions(&new_positions, &mut old_positions);
        new_positions.clear();

        for i in 0..old_positions.len() {
            new_positions.push(Vec::with_capacity(old_positions[i].len()));

            for j in 0..old_positions[i].len() {
                let old_state = old_positions[i][j];
                let new_state = match old_state {
                    Position::Floor => Position::Floor,
                    Position::Empty | Position::Occupied => {
                        let adjacent_occupied = get_adjacent_occupied(&old_positions, j, i);
                        match adjacent_occupied {
                            count if count == 0 => Position::Occupied,
                            count if count >= 4 => Position::Empty,
                            _ => old_state
                        }
                    }
                };

                new_positions[i].push(new_state);
            }
        }
    }

    count_all_occupied_seats(&new_positions)
}

fn fill_initial_positions<Grid: AsRef<[Row]>, Row: AsRef<[Position]>>(data: Grid, dest: &mut Vec::<Vec::<Position>>) {
    for row in data.as_ref().iter() {
        dest.push(Vec::with_capacity(row.as_ref().len()));
        let row_index = dest.len() - 1;

        for position in row.as_ref().iter() {
            dest[row_index].push(*position);
        }
    }
}

fn are_equal(a: &Vec::<Vec::<Position>>, b: &Vec::<Vec::<Position>>) -> bool {
    if a.len() != b.len() {
        return false;
    }

    for i in 0..a.len() {
        if a[i].len() != b[i].len() {
            return false;
        }

        for j in 0..a[i].len() {
            if a[i][j] != b[i][j] {
                return false;
            }
        }
    }

    true
}

#[allow(dead_code)]
fn print_positions(data: &Vec::<Vec::<Position>>) {
    for _ in 0..data[0].len() {
        print!("=");
    }
    println!("");

    for row in data.iter() {
        for position in row.iter() {
            match *position {
                Position::Floor => print!("."),
                Position::Empty => print!("L"),
                Position::Occupied => print!("#"),
            }
        }

        println!("");
    }

    for _ in 0..data[0].len() {
        print!("=");
    }
    println!("");
}

fn copy_positions(source: &Vec::<Vec::<Position>>, dest: &mut Vec::<Vec::<Position>>) {
    dest.clear();
    for i in 0..source.len() {
        dest.push(Vec::with_capacity(source[i].len()));

        for j in 0..source[i].len() {
            dest[i].push(source[i][j]);
        }
    }
}

fn get_adjacent_occupied(data: &Vec::<Vec::<Position>>, row: usize, col: usize) -> usize {
    let mut count = 0;

    if (row > 0) && (col > 0) {
        let up_left = data[col - 1][row - 1];
        if is_occupied(up_left) {
            count += 1;
        }
    }

    if col > 0 {
        let up = data[col - 1][row];
        if is_occupied(up) {
            count += 1;
        }
    }

    if (row < data[col].len() - 1) && (col > 0) {
        let up_right = data[col - 1][row + 1];
        if is_occupied(up_right) {
            count += 1;
        }
    }

    if row < data[col].len() - 1 {
        let right = data[col][row + 1];
        if is_occupied(right) {
            count += 1;
        }
    }

    if (row < data[col].len() - 1) && (col < data.len() - 1) {
        let bottom_right = data[col + 1][row + 1];
        if is_occupied(bottom_right) {
            count += 1;
        }
    }

    if col < data.len() - 1 {
        let bottom = data[col + 1][row];
        if is_occupied(bottom) {
            count += 1;
        }
    }

    if (row > 0) && (col < data.len() - 1) {
        let bottom_left = data[col + 1][row - 1];
        if is_occupied(bottom_left) {
            count += 1;
        }
    }

    if row > 0 {
        let left = data[col][row - 1];
        if is_occupied(left) {
            count += 1;
        }
    }

    count
}

fn is_occupied(position: Position) -> bool {
    match position {
        Position::Occupied => true,
        _ => false
    }
}

fn count_all_occupied_seats(data: &Vec::<Vec::<Position>>) -> usize {
    let mut count = 0;

    for row in data.iter() {
        for position in row.iter() {
            if is_occupied(*position) {
                count += 1;
            }
        }
    }

    count
}