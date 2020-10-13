use std::fs;

pub fn solution(filename: &String) {

    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let answer: u32 = contents
        .lines()
        .map(|x| { x.parse::<u32>().unwrap()})
        .map(fuel)
        .sum::<u32>()
        ;

    let answer_part_2: u32 = contents
        .lines()
        .map(|x| { x.parse::<u32>().unwrap()})
        .map(fuel2)
        .sum::<u32>()
        ;

    println!("part1:\n{:?}", answer);
    println!("part2:\n{:?}", answer_part_2);
}


fn fuel(mass: u32) -> u32 {
    (mass / 3).checked_sub(2).unwrap_or(0)
}

fn fuel2(mut mass: u32) -> u32 {
    let mut answer = 0;
    while mass > 0 {
        mass = fuel(mass);
        answer += mass
    }
    return answer
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day1_test1() {
        assert_eq!(2, fuel(12));
    }

    #[test]
    fn day1_test2() {
        assert_eq!(2, fuel(14));
    }

    #[test]
    fn day1_test3() {
        assert_eq!(654, fuel(1969));
    }

    #[test]
    fn day1_test4() {
        assert_eq!(33583, fuel(100756));
    }

    #[test]
    fn day1_part2_a() {
        assert_eq!(2, fuel2(14));
    }

    #[test]
    fn day1_part2_b() {
        assert_eq!(966, fuel2(1969));
    }

    #[test]
    fn day1_part2_c() {
        assert_eq!(50346, fuel2(100756));
    }
}