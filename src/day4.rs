use std::fs;

pub fn solution(filename: &String) {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let min_max: Vec<u32> = contents.split('-').map(|x| x.parse::<u32>().unwrap()).collect();
    let rules: Vec<&dyn PasswordRule> = vec![
        &IsCertainLength { length: 6 },
        &HasSameSequentialDigits {},
        &HasSequentiallyHigherNumbers {},
    ];
    println!("{:?}", password_generator(min_max[0], min_max[1], rules).len());
}

fn password_generator(min: u32, max: u32, rules: Vec<&dyn PasswordRule>) -> Vec<String>
{
    let mut passwords: Vec<String> = vec![];
    for password in min..max {
        let pass = password.to_string();
        if apply_rules(&pass, &rules)
        {
            passwords.push(pass);
        }
    }
    return passwords;
}

fn apply_rules(pass: &str, rules: &Vec<&dyn PasswordRule>) -> bool {
    rules.iter()
        .map(|rule| rule.is_valid(&String::from(pass)))
        .filter(|result| { result.clone() == false })
        .collect::<Vec<bool>>()
        .len() == 0
}


trait PasswordRule {
    fn is_valid(&self, password: &String) -> bool;
}

struct IsCertainLength {
    length: usize,
}

struct HasSameSequentialDigits {}

struct HasSequentiallyHigherNumbers {}

impl PasswordRule for IsCertainLength {
    fn is_valid(&self, password: &String) -> bool {
        password.chars().count() == self.length
    }
}

impl PasswordRule for HasSameSequentialDigits {
    fn is_valid(&self, password: &String) -> bool {
        let mut prev: char = ' ';
        for char in password.chars() {
            if char.eq(&prev) {
                return true;
            }
            prev = char.clone();
        }
        return false;
    }
}

impl PasswordRule for HasSequentiallyHigherNumbers {
    fn is_valid(&self, password: &String) -> bool {
        let mut highest: u32 = 0;
        for char in password.chars() {
            let digit: u32 = char.to_digit(10).unwrap();
            if digit < highest {
                return false;
            }
            highest = digit.clone();
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_rules() {
        let rules: Vec<&dyn PasswordRule> = vec![
            &IsCertainLength { length: 6 },
            &HasSameSequentialDigits {},
            &HasSequentiallyHigherNumbers {},
        ];

        assert_eq!(true, apply_rules(&"111111", &rules));
        assert_eq!(false, apply_rules(&"223450", &rules));
        assert_eq!(false, apply_rules(&"123789", &rules));
    }

    #[test]
    fn test_is_certain_length() {
        assert_eq!(true, IsCertainLength { length: 6 }.is_valid(&String::from("123456")));
        assert_eq!(false, IsCertainLength { length: 6 }.is_valid(&String::from("12345")));
        assert_eq!(false, IsCertainLength { length: 6 }.is_valid(&String::from("1234567")));
        assert_eq!(false, IsCertainLength { length: 1 }.is_valid(&String::from("123456")));
    }

    #[test]
    fn test_has_double_digits() {
        assert_eq!(true, HasSameSequentialDigits {}.is_valid(&String::from("112233")));
        assert_eq!(false, HasSameSequentialDigits {}.is_valid(&String::from("12345")));
        assert_eq!(false, HasSameSequentialDigits {}.is_valid(&String::from("1234567")));
        assert_eq!(false, HasSameSequentialDigits {}.is_valid(&String::from("123456")));
    }

    #[test]
    fn test_has_sequentially_higher_digits() {
        assert_eq!(true, HasSequentiallyHigherNumbers {}.is_valid(&String::from("112233")));
        assert_eq!(false, HasSequentiallyHigherNumbers {}.is_valid(&String::from("654321")));
        assert_eq!(false, HasSequentiallyHigherNumbers {}.is_valid(&String::from("1234576")));
    }
}