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
        boat_x_pos: i64,
        boat_y_pos: i64,
        waypoint_x_pos: i64,
        waypoint_y_pos: i64,
    }

    impl Boat {
        pub fn new() -> Boat {
            Boat {
                boat_x_pos: 0,
                boat_y_pos: 0,
                waypoint_x_pos: 10,
                waypoint_y_pos: -1,
            }
        }

        pub fn process_instruction(&mut self, instruction: &Instruction) {
            match instruction {
                Instruction::North(distance) => self.move_waypoint(Bearing::North, *distance),
                Instruction::South(distance) => self.move_waypoint(Bearing::South, *distance),
                Instruction::East(distance) => self.move_waypoint(Bearing::East, *distance),
                Instruction::West(distance) => self.move_waypoint(Bearing::West, *distance),
                Instruction::Left(degrees) => self.rotate_waypoint(TurnDirection::Left, *degrees),
                Instruction::Right(degrees) => self.rotate_waypoint(TurnDirection::Right, *degrees),
                Instruction::Forward(times) => self.move_forward(*times),
            }
        }

        pub fn get_coordinates(&self) -> (i64, i64) {
            (self.boat_x_pos, self.boat_y_pos)
        }

        fn move_waypoint(&mut self, direction: Bearing, distance: i64) {
            match direction {
                Bearing::North => self.waypoint_y_pos -= distance,
                Bearing::South => self.waypoint_y_pos += distance,
                Bearing::East => self.waypoint_x_pos += distance,
                Bearing::West => self.waypoint_x_pos -= distance
            }
        }

        fn move_forward(&mut self, move_times: i64) {
            self.boat_x_pos += self.waypoint_x_pos * move_times;
            self.boat_y_pos += self.waypoint_y_pos * move_times;
        }

        fn rotate_waypoint(&mut self, direction: TurnDirection, degrees: i32) {
            let mut degrees = degrees;

            while degrees != 0 {
                if degrees < 0 {
                    panic!("Non-90 degree turn!");
                }

                match direction {
                    TurnDirection::Left => {
                        let old_waypoint_x_pos = self.waypoint_x_pos;
                        let old_waypoint_y_pos = self.waypoint_y_pos;

                        self.waypoint_y_pos = -old_waypoint_x_pos;
                        self.waypoint_x_pos = old_waypoint_y_pos;
                    },
                    TurnDirection::Right => {
                        let old_waypoint_x_pos = self.waypoint_x_pos;
                        let old_waypoint_y_pos = self.waypoint_y_pos;

                        self.waypoint_y_pos = old_waypoint_x_pos;
                        self.waypoint_x_pos = -old_waypoint_y_pos;
                    },
                }

                degrees -= 90;
            }
        }
    }
}
