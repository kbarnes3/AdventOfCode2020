#[allow(unused_imports)]
use day12_rain_risk_common::{Instruction, SAMPLE_DATA, REAL_DATA};
use boat::Boat;

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work(data: &[Instruction]) -> i64 {
    let mut boat = Boat::new();

    for instruction in data.iter() {
        boat.process_instruction(instruction);
    }
    
    let coordinates = boat.get_coordinates();
    coordinates.0.abs() + coordinates.1.abs()
}

mod boat {
    use day12_rain_risk_common::Instruction;

    #[derive(Copy, Clone)]
    enum Bearing {
        North,
        South,
        East,
        West
    }

    #[derive(Copy, Clone)]
    enum TurnDirection {
        Left,
        Right
    }

    pub struct Boat {
        bearing: Bearing,
        boat_x_pos: i64,
        boat_y_pos: i64,
        waypoint_x_pos: i64,
        waypoint_y_pos: i64,
    }

    impl Boat {
        pub fn new() -> Boat {
            Boat {
                bearing: Bearing::East,
                boat_x_pos: 0,
                boat_y_pos: 0,
                waypoint_x_pos: 10,
                waypoint_y_pos: -1,
            }
        }

        pub fn process_instruction(&mut self, instruction: &Instruction) {
            match instruction {
                Instruction::North(distance) => self.move_direction(Bearing::North, *distance),
                Instruction::South(distance) => self.move_direction(Bearing::South, *distance),
                Instruction::East(distance) => self.move_direction(Bearing::East, *distance),
                Instruction::West(distance) => self.move_direction(Bearing::West, *distance),
                Instruction::Left(degrees) => self.turn_ship(TurnDirection::Left, *degrees),
                Instruction::Right(degrees) => self.turn_ship(TurnDirection::Right, *degrees),
                Instruction::Forward(distance) => self.move_forward(*distance),
            }
        }

        pub fn get_coordinates(&self) -> (i64, i64) {
            (self.boat_x_pos, self.boat_y_pos)
        }

        fn move_direction(&mut self, direction: Bearing, distance: i64) {
            match direction {
                Bearing::North => self.boat_y_pos -= distance,
                Bearing::South => self.boat_y_pos += distance,
                Bearing::East => self.boat_x_pos -= distance,
                Bearing::West => self.boat_x_pos += distance
            }
        }

        fn move_forward(&mut self, distance: i64) {
            self.move_direction(self.bearing, distance);
        }

        fn turn_ship(&mut self, direction: TurnDirection, degrees: i32) {
            let mut degrees = degrees;

            while degrees != 0 {
                if degrees < 0 {
                    panic!("Non-90 degree turn!");
                }

                match (direction, self.bearing) {
                    (TurnDirection::Left, Bearing::North) => self.bearing = Bearing::West,
                    (TurnDirection::Left, Bearing::East) => self.bearing = Bearing::North,
                    (TurnDirection::Left, Bearing::South) => self.bearing = Bearing::East,
                    (TurnDirection::Left, Bearing::West) => self.bearing = Bearing::South,
                    (TurnDirection::Right, Bearing::North) => self.bearing = Bearing::East,
                    (TurnDirection::Right, Bearing::East) => self.bearing = Bearing::South,
                    (TurnDirection::Right, Bearing::South) => self.bearing = Bearing::West,
                    (TurnDirection::Right, Bearing::West) => self.bearing = Bearing::North,
                }

                degrees -= 90;
            }
        }
    }
}
