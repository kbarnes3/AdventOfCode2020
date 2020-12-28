use std::cmp::{ max, min };
use std::collections::HashMap;

#[allow(unused_imports)]
use day17_conway_cubes_common::{Cube, CYCLES, SAMPLE_DATA, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

#[derive(PartialEq, Eq, Hash)]
struct Vector3 {
    x: i64,
    y: i64,
    z: i64
}

fn do_work<Plane: AsRef<[Row]>, Row: AsRef<[Cube]>>(data: Plane) -> usize {
    let mut dimension = HashMap::new();
    let mut min_coord = Vector3 { x: -1, y: -1, z: -1 };
    let mut max_coord = Vector3 { 
        x: data.as_ref()[0].as_ref().len() as i64,
        y: data.as_ref().len() as i64,
        z: 1};

    for (x, row) in data.as_ref().iter().enumerate() {
        let x = x as i64;
        for (y, cube) in row.as_ref().iter().enumerate() {
            let y = y as i64;
            let coordinates = Vector3 { x, y , z: 0 };

            dimension.insert(coordinates, *cube);
        }
    }

    for _ in 0..CYCLES {
        let mut new_dimension = HashMap::new();
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        let mut min_z = 0;
        let mut max_z = 0;

        for x in min_coord.x..=max_coord.x {
            for y in min_coord.y..=max_coord.y {
                for z in min_coord.z..=max_coord.z {
                    let coordinates = Vector3 { x, y, z };
                    let cube = dimension.get(&coordinates).unwrap_or(&Cube::Inactive);
                    let neighbors = count_neighbors(&dimension, &coordinates);

                    let new_cube = match cube {
                        Cube::Active => {
                            if (neighbors == 2) || (neighbors == 3) {
                                Cube::Active
                            } else {
                                Cube::Inactive
                            }
                        },
                        Cube::Inactive => {
                            if neighbors == 3 {
                                Cube::Active
                            } else {
                                Cube::Inactive
                            }
                        }
                    };

                    if let Cube::Active = new_cube {
                        min_x = min(min_x, coordinates.x);
                        max_x = max(max_x, coordinates.x);
                        min_y = min(min_y, coordinates.y);
                        max_y = max(max_y, coordinates.y);
                        min_z = min(min_z, coordinates.z);
                        max_z = max(max_z, coordinates.z);

                        new_dimension.insert(coordinates, new_cube);
                    }
                }
            }
        }

        dimension = new_dimension;
        min_coord = Vector3 { x: min_x-1, y: min_y-1, z: min_z-1 };
        max_coord = Vector3 { x: max_x+1, y: max_y+1, z: max_z+1 };
    }

    let mut active_cubes = 0;
    for (_coordinates, cube) in dimension.iter() {
        active_cubes += match cube {
            Cube::Active => 1,
            Cube::Inactive => 0,
        }
    }
    
    active_cubes
}

fn count_neighbors(dimension: &HashMap::<Vector3, Cube>, coordinates: &Vector3) -> usize {
    let mut neighbors_count = 0;
    for x in (coordinates.x-1)..=(coordinates.x+1) {
        for y in (coordinates.y-1)..=(coordinates.y+1) {
            for z in (coordinates.z-1)..=(coordinates.z+1) {
                let neighbor = Vector3 { x, y, z };
                if neighbor != *coordinates {
                    let cube = dimension.get(&neighbor).unwrap_or(&Cube::Inactive);
                    neighbors_count += match cube {
                        Cube::Active => 1,
                        Cube::Inactive => 0
                    };
                }
            }
        }
    }

    neighbors_count
}
