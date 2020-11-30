use std::collections::HashMap;
use std::fmt::Write;
use std::fs;

pub fn solution(filename: &String) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut prog = Program::from(contents.as_str());
    println!("a: {:?}", prog.execute("1"));

    let mut prog = Program::from(contents.as_str());
    println!("b: {:?}", prog.execute("5"));
}

struct Program {
    opcodes: HashMap<usize, i32>,
    index: usize,
    input: String,
    output: String,
}

impl Program {
    fn new(str: &str) -> Program {
        let codes: Vec<i32> = str.split(",").map(|str| str.parse().unwrap()).collect();
        let mut opcodes = HashMap::new();
        let mut index: usize = 0;
        for opcode in codes {
            opcodes.insert(index, opcode);
            index = index + 1;
        }

        return Program {
            input: String::new(),
            output: String::new(),
            opcodes,
            index: 0,
        };
    }

    fn execute(&mut self, input: &str) -> String {
        self.input.write_str(input).unwrap();
        loop {
            let opcode = self.opcodes[&self.index];
            match opcode % 100 {
                1 => Addition::apply(self),
                2 => Multiplication::apply(self),
                3 => Input::apply(self),
                4 => Output::apply(self),
                5 => Jump::apply(self, true),
                6 => Jump::apply(self, false),
                7 => Compare::apply(self, '<'),
                8 => Compare::apply(self, '='),
                99 => return self.output.clone(),
                _ => panic!("unknown opcode"),
            }
        }
    }

    fn next(&mut self) -> i32 {
        self.index = self.index + 1;
        return self.opcodes[&self.index];
    }
}

trait Operation {
    fn execute(program: &Program);
}

#[derive(Debug)]
struct ArgumentModes {
    modes: Vec<u32>,
}

impl ArgumentModes {
    fn new(opcode: i32) -> ArgumentModes {
        let opcode_string = opcode.to_string();
        if opcode_string.len() <= 2 {
            return ArgumentModes { modes: vec![] };
        }

        let modes: Vec<u32> = opcode_string
            .get(..opcode_string.len() - 2)
            .unwrap()
            .chars()
            .rev()
            .map(|char| char.to_digit(10).unwrap())
            .collect();

        return ArgumentModes { modes };
    }

    fn get_mode(&self, idx: usize) -> u32 {
        return self.modes.iter().nth(idx).unwrap_or(&0).clone();
    }

    fn get_value(&self, arg: usize, idx: usize, opcodes: &HashMap<usize, i32>) -> i32 {
        match self.get_mode(arg) {
            0 => opcodes[&(opcodes[&idx] as usize)],
            1 => opcodes[&idx],
            _ => panic!("cant find mode!?"),
        }
    }
}

struct Multiplication {}

struct Addition {}

struct Input {}

struct Output {}

struct Jump {}

struct Compare {}

impl Multiplication {
    fn apply(program: &mut Program) {
        let arg_modes = ArgumentModes::new(program.opcodes[&program.index]);
        program.next();
        let arg_a = arg_modes.get_value(0, program.index, &program.opcodes);
        program.next();
        let arg_b = arg_modes.get_value(1, program.index, &program.opcodes);
        program.next();
        let position = program.opcodes[&program.index];

        program.opcodes.insert(position as usize, arg_a * arg_b);
        program.next();
    }
}

impl Addition {
    fn apply(program: &mut Program) {
        let arg_modes = ArgumentModes::new(program.opcodes[&program.index]);
        program.next();
        let arg_a = arg_modes.get_value(0, program.index, &program.opcodes);
        program.next();
        let arg_b = arg_modes.get_value(1, program.index, &program.opcodes);
        program.next();
        let position = program.opcodes[&program.index];

        program.opcodes.insert(position as usize, arg_a + arg_b);
        program.next();
    }
}

impl Input {
    fn apply(program: &mut Program) {
        let arg_modes = ArgumentModes::new(program.opcodes[&program.index]);
        program.next();
        let position = program.opcodes[&program.index];

        let input = program.input.parse();
        if input.is_err() {
            panic!("input required");
        }

        println!(
            "idx: {:?} op: Input value: {:?} write to position:{:?} {:?}",
            program.index - 1,
            input,
            position,
            arg_modes
        );

        program.opcodes.insert(position as usize, input.unwrap());
        program.next();
    }
}

impl Output {
    fn apply(program: &mut Program) {
        let arg_modes = ArgumentModes::new(program.opcodes[&program.index]);
        program.next();
        let position = match arg_modes.get_mode(0) {
            0 => program.opcodes[&program.index],
            1 => program.index as i32,
            _ => panic!("wrong mode"),
        };

        program.next();

        let value: i32 = program.opcodes[&(position as usize)];
        println!(
            "idx: {:?} op: Output value: {:?} from position:{:?} {:?}",
            program.index - 1,
            value,
            position,
            arg_modes
        );

        program
            .output
            .write_str(format!("{}\n", value.to_string()).as_str())
            .unwrap();
    }
}

impl Jump {
    fn apply(program: &mut Program, is_not_zero: bool) {
        let arg_modes = ArgumentModes::new(program.opcodes[&program.index]);
        program.next();
        let arg_a = arg_modes.get_value(0, program.index, &program.opcodes);
        program.next();
        let arg_b = arg_modes.get_value(1, program.index, &program.opcodes);
        program.next();

        println!(
            "idx: {:?} op: Jump if zero {:?} a: {:?} jump to idx: {:?} {:?}",
            program.index - 1,
            is_not_zero,
            arg_a,
            arg_b,
            arg_modes
        );

        if is_not_zero == true && arg_a != 0 || is_not_zero == false && arg_a == 0 {
            program.index = arg_b as usize;
        }
    }
}

macro_rules! ternary {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {
            $v
        } else {
            $v1
        }
    };
}

impl Compare {
    fn apply(program: &mut Program, operator: char) {
        let arg_modes = ArgumentModes::new(program.opcodes[&program.index]);
        program.next();
        let arg_a = arg_modes.get_value(0, program.index, &program.opcodes);
        program.next();
        let arg_b = arg_modes.get_value(1, program.index, &program.opcodes);
        program.next();
        let position = program.opcodes[&program.index];

        let value = match operator {
            '<' => ternary!(arg_a < arg_b, 1, 0),
            '=' => ternary!(arg_a == arg_b, 1, 0),
            _ => panic!("unknown compare mode"),
        };

        println!(
            "idx: {:?} op: Compare mode {:?} a: {:?} b: {:?} write {:?} to pos {:?} {:?}",
            program.index - 1,
            operator,
            arg_a,
            arg_b,
            value,
            position,
            arg_modes
        );

        program.opcodes.insert(position as usize, value);
        program.next();
    }
}

impl From<&str> for Program {
    fn from(str: &str) -> Self {
        return Program::new(str);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut prog = Program::from("1002,4,3,4,33");
        prog.execute("");
        assert_eq!(99, prog.opcodes[&4]);

        let mut prog = Program::from("2,4,2,0,4,0,99");
        assert_eq!("8\n", prog.execute(""));
    }

    #[test]
    fn test_input() {
        let mut prog = Program::from("3,0,4,0,99");
        assert_eq!(String::from("15\n"), prog.execute("15"));
        println!("{:?}", prog.opcodes);
    }

    #[test]
    fn test_program_day2() {
        let mut prog = Program::from("1,0,0,0,99");
        prog.execute("");
        assert_eq!(2, prog.opcodes[&(0 as usize)]);

        let mut prog = Program::from("2,3,0,3,99");
        prog.execute("");
        assert_eq!(6, prog.opcodes[&(3 as usize)]);

        let mut prog = Program::from("2,4,4,5,99,0");
        prog.execute("");
        assert_eq!(9801, prog.opcodes[&(5 as usize)]);

        let mut prog = Program::from("1,1,1,4,99,5,6,0,99");
        prog.execute("");
        assert_eq!(30, prog.opcodes[&(0 as usize)]);
    }

    #[test]
    fn test_arguments() {
        assert_eq!(vec![0, 1], ArgumentModes::new(1002).modes);
        assert_eq!(vec![1, 1], ArgumentModes::new(01102).modes);

        let arg_modes = ArgumentModes::new(01102);
        assert_eq!(1, arg_modes.get_mode(0));
        assert_eq!(1, arg_modes.get_mode(1));
        assert_eq!(0, arg_modes.get_mode(2));

        let arg_modes = ArgumentModes::new(10102);
        assert_eq!(1, arg_modes.get_mode(0));
        assert_eq!(0, arg_modes.get_mode(1));
        assert_eq!(1, arg_modes.get_mode(2));
    }

    #[test]
    fn test_day5_b_compare() {
        // positional mode, equals
        let mut prog = Program::from("3,9,8,9,10,9,4,9,99,-1,8");
        assert_eq!("1\n", prog.execute("8"));

        let mut prog = Program::from("3,9,8,9,10,9,4,9,99,-1,8");
        assert_eq!("0\n", prog.execute("1"));

        // immediate mode, equals
        let mut prog = Program::from("3,3,1108,-1,8,3,4,3,99");
        assert_eq!("1\n", prog.execute("8"));

        let mut prog = Program::from("3,3,1108,-1,8,3,4,3,99");
        assert_eq!("0\n", prog.execute("1"));

        // positional mode, less then
        let mut prog = Program::from("3,9,7,9,10,9,4,9,99,-1,8");
        assert_eq!("1\n", prog.execute("7"));

        let mut prog = Program::from("3,9,7,9,10,9,4,9,99,-1,8");
        assert_eq!("0\n", prog.execute("9"));

        // immediate mode, less then
        let mut prog = Program::from("3,3,1107,-1,8,3,4,3,99");
        assert_eq!("1\n", prog.execute("7"));

        let mut prog = Program::from("3,3,1107,-1,8,3,4,3,99");
        assert_eq!("0\n", prog.execute("9"));
    }

    #[test]
    fn test_day5_b_jump() {
        let mut prog = Program::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        assert_eq!("0\n", prog.execute("0"));
        let mut prog = Program::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        assert_eq!("1\n", prog.execute("5"));
    }

    #[test]
    fn test_day5_b() {
        let mut prog = Program::from("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        assert_eq!("999\n", prog.execute("7"));

        let mut prog = Program::from("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        assert_eq!("1000\n", prog.execute("8"));

        let mut prog = Program::from("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        assert_eq!("1001\n", prog.execute("9"));
    }
}
