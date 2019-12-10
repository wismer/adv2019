use nom::{alt, char, named, map, pair, parse_to};
use std::fs;
use failure::Error;

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down
}

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub struct Instruction {
    pub direction: Direction,
    pub value: u32,
}

named!(
    pub dir<&[u8], char>,
    alt!(
        char!('U') |
        char!('D') |
        char!('L') |
        char!('R')
    )
);

named!(
    pub instruction<&[u8], Instruction>,
    map!(
        pair!(
            dir,
            parse_to!(u32)
        ),
        |(c, value)| {
            let direction = match c {
                'U' => Direction::Up,
                'L' => Direction::Left,
                'R' => Direction::Right,
                'D' => Direction::Down,
                _ => unreachable!("unpossible")
            };

            Instruction {
                direction, value
            }
        }
    )
);

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
struct Pathway {
    from: Point,
    to: Point,
    direction: Direction,
}

fn day_3_part_one() -> Result<(), Error> {
    let input = fs::read_to_string("./day3.txt")?;
    let mut lines = input.lines();
    let first_line: Vec<Instruction> = lines.next().unwrap().split(",").map(|i| {
        instruction(i.as_bytes()).unwrap().1
    }).collect();
    let second_line: Vec<Instruction> = lines.next().unwrap().split(",").map(|i| {
        instruction(i.as_bytes()).unwrap().1
    }).collect();
    // let mut intersection_points: Vec<Point> = vec![];
    
    let mut first_path: Vec<Pathway> = vec![];
    let mut second_path: Vec<Pathway> = vec![];
    let mut cursor = Point { x: 0, y: 0 };
    let mut xmax = 0;
    let mut ymax = 0;
    for p in first_line {
        let destination = match p.direction {
            Direction::Down => Point { x: cursor.x, y: cursor.y - p.value as i32 },
            Direction::Up => Point { x: cursor.x, y: cursor.y + p.value as i32 },
            Direction::Left => Point { x: cursor.x - p.value as i32, y: cursor.y },
            Direction::Right => Point { x: cursor.x + p.value as i32, y: cursor.y },
        };

        first_path.push(Pathway { from: cursor, to: destination, direction: p.direction });
        if destination.x > xmax {
            xmax = destination.x;
        }
        cursor = destination;
    }

    println!("{:#?}", xmax);

    cursor = Point { x: 0, y: 0 };

    for p in second_line {
        let destination = match p.direction {
            Direction::Down => Point { x: cursor.x, y: cursor.y - p.value as i32 },
            Direction::Up => Point { x: cursor.x, y: cursor.y + p.value as i32 },
            Direction::Left => Point { x: cursor.x - p.value as i32, y: cursor.y },
            Direction::Right => Point { x: cursor.x + p.value as i32, y: cursor.y },
        };

        second_path.push(Pathway { from: cursor, to: destination, direction: p.direction });
        cursor = destination;
    }

    // for fpath in first_path {
    //     for spath in second_path.clone() {
    //         println!("{:#?}", intersection_point(&fpath, &spath))
    //     }
    // }




    // if let Some(first_line) = lines.next() {
    //     first_line
    // } else {
    //     unreachable!()
    // };

    // if let Some(second_line) = lines.next() {
    //     second_line
    // } else {
    //     unreachable!()
    // };

    
    // for line in input.lines() {
    //     let bytes = line.split(",").map(|i| instruction(i.as_bytes()));
    //     for byte in bytes {
    //         println!("{:#?}", byte);
    //     }
    // }

    Ok(())
}
