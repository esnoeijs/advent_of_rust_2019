use std::fs;
use std::cell::RefCell;
use std::collections::HashMap;

pub fn solution(filename: &String) {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let wires: Vec<Wire> = contents.lines().map(|str| { Wire::from(str) }).collect();
    let wire_a = wires[0].clone();
    let wire_b = wires[1].clone();

    println!("{:?}", intersect(&wire_a, &wire_b)
        .iter()
        .map(|x| { manhattan_distance([0, 0], [x.x, x.y]) })
        .min()
        .unwrap());
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Directions { U, D, R, L }

#[derive(Debug, PartialEq)]
struct Move {
    direction: Directions,
    steps: u32,
}

#[derive(Clone, Debug, PartialEq)]
struct Wire {
    route: RefCell<HashMap<Pos, Pos>>,
    current: Pos,
}

impl Wire {
    fn new() -> Wire
    {
        let mut map = HashMap::new();
        map.insert(Pos::start(), Pos::start());
        return Wire {
            route: RefCell::new(map.clone()),
            current: Pos::start(),
        };
    }

    fn new_from_route(moves: Vec<Move>) -> Wire
    {
        let mut wire = Wire::new();
        wire.apply_moves(moves);
        return wire;
    }

    fn apply_moves(&mut self, moves: Vec<Move>)
    {
        for mov in moves {
            for _ in 0..mov.steps {
                let pos = self.current.clone();
                let new_pos = match mov.direction {
                    Directions::U => Pos { x: pos.x, y: pos.y + 1 },
                    Directions::D => Pos { x: pos.x, y: pos.y - 1 },
                    Directions::R => Pos { x: pos.x + 1, y: pos.y },
                    Directions::L => Pos { x: pos.x - 1, y: pos.y },
                };

                self.route.borrow_mut().insert(new_pos.clone(), new_pos.clone());
                self.current = new_pos.clone();
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn start() -> Pos
    {
        return Pos { x: 0, y: 0 };
    }
}

impl From<&str> for Wire {
    fn from(str: &str) -> Self
    {
        Wire::new_from_route(str.split(',').map(|x| { Move::from(x) }).collect())
    }
}

impl From<&str> for Move {
    fn from(str: &str) -> Self
    {
        let direction: Directions;
        match str.chars().nth(0).unwrap() {
            'U' => direction = Directions::U,
            'D' => direction = Directions::D,
            'L' => direction = Directions::L,
            'R' => direction = Directions::R,
            _ => panic!("unknown direction")
        }

        let steps = (str[1..]).parse::<u32>();
        if steps.is_err() {
            panic!("not a number");
        }

        return Move { direction, steps: steps.unwrap() };
    }
}

fn intersect(wire_a: &Wire, wire_b: &Wire) -> Vec<Pos>
{
    let mut intersecting_positions: Vec<Pos> = vec![];
    for pos in wire_a.route.borrow().keys() {
        if wire_b.route.borrow().contains_key(pos) {
            if pos.x != 0 || pos.y != 0 {
                intersecting_positions.push(pos.clone())
            }
        }
    }
    return intersecting_positions;
}

fn manhattan_distance(start: [i32; 2], end: [i32; 2]) -> i32
{
    return (start[0] - end[0]).abs() + (start[1] - end[1]).abs();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let w1 = Wire::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let w2 = Wire::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(135, intersect(&w1, &w2).iter().map(|x| { manhattan_distance([0, 0], [x.x, x.y]) }).min().unwrap());

        let w1 = Wire::from("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let w2 = Wire::from("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(159, intersect(&w1, &w2).iter().map(|x| { manhattan_distance([0, 0], [x.x, x.y]) }).min().unwrap());
    }

    #[test]
    fn string_to_move_mapping() {
        assert_eq!(Move { direction: Directions::U, steps: 1 }, Move::from("U1"));
    }
}