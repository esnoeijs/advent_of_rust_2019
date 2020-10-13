use std::fs;

pub fn solution(filename: &String) {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let sequence: Vec<u32> = contents
        .split(',')
        .map(|x| { x.parse::<u32>().unwrap()})
        .collect()
    ;

    println!("{:?}", calculate_operations(sequence));
}


fn calculate_operations(mut sequence: Vec<u32>) -> Vec<u32>
{
    for mut index in 0..sequence.len() {

        let idx = index * 4;

        let operation: u32 = sequence.get(idx).copied().unwrap_or(99);

        if operation == 99 {
            break;
        }

        let a = sequence[idx + 1] as usize;
        let b = sequence[idx + 2] as usize;
        let pos = sequence[idx + 3] as usize;

        match operation {
            1 => sequence[pos] = sequence[a] + sequence[b],
            2 => sequence[pos] = sequence[a] * sequence[b],
            _ => break
        }
    };

    return sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_test1() {
        assert_eq!(2, calculate_operations(vec![1,0,0,0,99])[0]);
    }

    #[test]
    fn day1_test2() {
        assert_eq!(6, calculate_operations(vec![2,3,0,3,99])[3]);
    }

    #[test]
    fn day1_test3() {
        assert_eq!(9801, calculate_operations(vec![2,4,4,5,99,0])[5]);
    }

    #[test]
    fn day1_test4() {
        assert_eq!(30, calculate_operations(vec![1,1,1,4,99,5,6,0,99])[0]);
    }
}

