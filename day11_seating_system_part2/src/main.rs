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
                        let adjacent_occupied = get_visible_occupied(&old_positions, j, i);
                        match adjacent_occupied {
                            count if count == 0 => Position::Occupied,
                            count if count >= 5 => Position::Empty,
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

fn get_visible_occupied(data: &Vec::<Vec::<Position>>, row: usize, col: usize) -> usize {
    let mut count = 0;
    const DIRECTIONS: [(RowDirection, ColDirection); 8] = [
        (RowDirection::Left, ColDirection::Up),
        (RowDirection::None, ColDirection::Up),
        (RowDirection::Right, ColDirection::Up),
        (RowDirection::Right, ColDirection::None),
        (RowDirection::Right, ColDirection::Down),
        (RowDirection::None, ColDirection::Down),
        (RowDirection::Left, ColDirection::Down),
        (RowDirection::Left, ColDirection::None)
    ];

    for direction in DIRECTIONS.iter() {
        if is_visible_occupied_in_direction(data, row, col, direction.0, direction.1) {
            count += 1;
        }
    }

    count
}

#[derive(Copy, Clone)]
enum RowDirection {
    None,
    Left,
    Right,
}

#[derive(Copy, Clone)]
enum ColDirection {
    None,
    Up,
    Down,
}

fn is_visible_occupied_in_direction(
    data: &Vec::<Vec::<Position>>,
    start_row: usize,
    start_col: usize,
    row_direction: RowDirection,
    col_direction: ColDirection
) -> bool {
    let mut row = start_row;
    let mut col = start_col;

    loop {
        match col_direction {
            ColDirection::None => col = col,
            ColDirection::Up => {
                if col == 0 {
                    return false;
                }
                col -= 1;
            },
            ColDirection::Down => {
                if col >= data.len() - 1 {
                    return false;
                }
                col += 1;
            }
        }

        match row_direction {
            RowDirection::None => row = row,
            RowDirection::Left => {
                if row == 0 {
                    return false;
                }
                row -= 1;
            },
            RowDirection::Right => {
                if row >= data[col].len() - 1 {
                    return false;
                }
                row += 1;
            },
        };

        let position = data[col][row];
        match position {
            Position::Floor => (),
            Position::Empty => return false,
            Position::Occupied => return true,
        }

    }
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