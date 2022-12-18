use crate::utils;
use crate::algebra::Matrix3D;

use std::{path::Path, collections::HashSet};

#[derive(Clone,Copy,Eq,PartialEq,PartialOrd,Hash)]
struct Point3d(i32,i32,i32);

type Space = HashSet<Point3d>;

fn parse_space<P>(filename : P) -> Space
where P : AsRef<Path> {
    let mut space : Space = HashSet::new();

    if let Ok(lines) = utils::read_lines(filename) {       
        for line in lines {
            if let Ok(point) = line {
                let parts : Vec<&str> = point.split(',').collect();
                if parts.len() == 3 {
                    let p = Point3d(
                        parts[0].parse::<i32>().unwrap(),
                        parts[1].parse::<i32>().unwrap(),
                        parts[2].parse::<i32>().unwrap());

                    space.insert(p);
                }
                else {
                    panic!("Unexpected format!");
                }
            }
        }
    }    
    space
}

fn count_exposed_sides(space : &Space) -> u32 {
    let mut sides = 0;

    let dx : [i32; 6] = [-1, 1, 0,  0, 0,  0];
    let dy : [i32; 6] = [ 0, 0, 1, -1, 0,  0];
    let dz : [i32; 6] = [ 0, 0, 0,  0, 1, -1];

    for point in space {
        for i in 0..6 {
            let adjacent = Point3d(point.0 + dx[i], point.1 + dy[i], point.2 + dz[i]);

            if !space.contains(&adjacent){
                sides += 1;
            }
        }
    }

    sides
}

const AIR : u8 = 0;
const LAVA : u8 = 1;
const INNER_AIR : u8 = 2;

fn adjacent_voxels(voxels: &Matrix3D<u8>, x : usize, y : usize, z : usize) -> usize {
    let mut sides = 0;

    if x == 0 || voxels.element_at(x - 1, y, z).unwrap() == AIR {
        sides += 1;
    }

    if x == voxels.size_x-1 || voxels.element_at(x + 1, y, z).unwrap() == AIR {
        sides += 1;
    }

    if y == 0 || voxels.element_at(x, y - 1, z).unwrap() == AIR {
        sides += 1;
    }

    if y == voxels.size_y-1 || voxels.element_at(x, y + 1, z).unwrap() == AIR {
        sides += 1;
    }

    if z == 0 || voxels.element_at(x, y, z - 1).unwrap() == AIR {
        sides += 1;
    }

    if z == voxels.size_z-1 || voxels.element_at(x, y, z + 1).unwrap() == AIR {
        sides += 1;
    }

    sides
}

fn count_exterior_surface_area(space : &Space) -> usize {
    let mut area = 0;

    //let minx = space.iter().fold(0i32, |min, &val| if val.0 < min{ val.0 } else{ min });
    let maxx = space.iter().fold(0i32, |max, &val| if val.0 > max{ val.0 } else{ max });

    //let miny = space.iter().fold(0i32, |min, &val| if val.1 < min{ val.1 } else{ min });
    let maxy = space.iter().fold(0i32, |max, &val| if val.1 > max{ val.1 } else{ max });

    //let minz = space.iter().fold(0i32, |min, &val| if val.2 < min{ val.2 } else{ min });
    let maxz = space.iter().fold(0i32, |max, &val| if val.2 > max{ val.2 } else{ max });

    let umaxx = usize::try_from(maxx).unwrap();
    let umaxy = usize::try_from(maxy).unwrap();
    let umaxz = usize::try_from(maxz).unwrap();

    // build a space of voxels
    let mut voxels : Matrix3D<u8> = Matrix3D::new(umaxx+1, umaxy+1, umaxz+1, AIR);

    // set the lava voxels
    for v in space {
        voxels.set_at(
            usize::try_from(v.0).unwrap(), 
            usize::try_from(v.1).unwrap(), 
            usize::try_from(v.2).unwrap(), 
            LAVA);
    }

    // set all the voxels that are not on the margin as inner air pockets
    for x in 1..umaxx {
        for y in 1..umaxy {
            for z in 1..umaxz {
                if let Some(v) = voxels.element_at(x, y, z) {
                    if v == AIR {
                        voxels.set_at(x, y, z, INNER_AIR);
                    }
                }
            }
        }
    }

    // all inner air pockets that are adjacent to outside air must be outside air
    loop {
        let mut changed = false;

        for x in 0..=umaxx {
            for y in 0..=umaxy {
                for z in 0..=umaxz {
                    if let Some(e) = voxels.element_at(x, y, z) {
                        if e == INNER_AIR && adjacent_voxels(&voxels, x, y, z) > 0 {
                            voxels.set_at(x, y, z, AIR);
                            changed = true;
                        }
                    }
                }
            }
        }

        if !changed {
            break;
        }
    }

    // count the voxels sides between AIR and LAVA
    for x in 0..=umaxx {
        for y in 0..=umaxy {
            for z in 0..=umaxz {
                if let Some(v) = voxels.element_at(x, y, z) {
                    if v == LAVA {
                        area += adjacent_voxels(&voxels, x, y, z);
                    }
                }
            }
        }
    }

    area
}

pub fn execute() {
    println!("=== puzzle 18 ===");

    let test_space = parse_space("./data/input18test.txt");
    let test_count = count_exposed_sides(&test_space);
    assert_eq!(64, test_count);

    let space = parse_space("./data/input18.txt");
    let count = count_exposed_sides(&space);
    println!("sides={}", count);

    let test_area = count_exterior_surface_area(&test_space);
    assert_eq!(58, test_area);

    let area = count_exterior_surface_area(&space);
    println!("area={}", area);

    println!();
}