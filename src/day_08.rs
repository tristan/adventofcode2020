use adventofcode2020::{ReadError, parse_input_file};

#[derive(Debug)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

struct Program(Vec<Instruction>);

impl From<Vec<Instruction>> for Program {
    fn from(instructions: Vec<Instruction>) -> Program {
        Program(instructions)
    }
}

impl Program {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn instruction_at<'a>(&'a self, pointer: usize) -> Option<&'a Instruction> {
        self.0.get(pointer)
    }

    fn swap_instruction<'a>(&'a mut self, pointer: usize) -> Result<(), ()> {
        if pointer < self.0.len() {
            let new = match self.0[pointer] {
                Instruction::Acc(_) => return Err(()),
                Instruction::Jmp(val) => Instruction::Nop(val),
                Instruction::Nop(val) => Instruction::Jmp(val),
            };
            let _ = std::mem::replace(&mut self.0[pointer], new);
            Ok(())
        } else {
            Err(())
        }
    }
}

fn parse_instruction(line_no: usize, line: String) -> Result<Instruction, ReadError> {
    let val = line[4..].parse::<i32>()
        .map_err(|_| ReadError::ParseError(line_no, line.clone()))?;
    match &line[..3] {
        "acc" => {
            Ok(Instruction::Acc(val))
        },
        "jmp" => {
            Ok(Instruction::Jmp(val))
        },
        "nop" => Ok(Instruction::Nop(val)),
        _ => Err(ReadError::ParseError(line_no, line))
    }
}

struct BootLoader<'a> {
    program: &'a Program,
    acc: i32,
    pointer: usize,
    instructions_called: Vec<bool>
}

#[derive(Debug)]
enum BootLoaderError {
    InstructionOverflow(usize),
    InvalidJump(usize, i32),
    LoopDetected(usize, i32),
}

impl<'a> BootLoader<'a> {
    fn new(program: &'a Program) -> BootLoader<'a> {
        BootLoader {
            program,
            acc: 0,
            pointer: 0,
            instructions_called: vec![false; program.len()]
        }
    }

    fn step(&mut self) -> Result<(), BootLoaderError> {
        if let Some(instruction) = self.program.instruction_at(self.pointer) {
            if self.instructions_called[self.pointer] {
                Err(BootLoaderError::LoopDetected(self.pointer, self.acc))
            } else {
                self.instructions_called[self.pointer] = true;
                match instruction {
                    Instruction::Acc(val) => {
                        self.acc += val;
                        self.pointer += 1;
                        Ok(())
                    },
                    Instruction::Jmp(val) => {
                        let pointer = self.pointer as i32 + val;
                        if pointer < 0 {
                            Err(BootLoaderError::InvalidJump(self.pointer, *val))
                        } else {
                            self.pointer = pointer as usize;
                            Ok(())
                        }
                    },
                    Instruction::Nop(_) => {
                        self.pointer += 1;
                        Ok(())
                    },
                }
            }
        } else {
            Err(BootLoaderError::InstructionOverflow(self.pointer))
        }
    }

    fn boot(&mut self) -> Result<i32, BootLoaderError> {
        loop {
            self.step()?;
            if self.pointer == self.program.len() {
                return Ok(self.acc);
            }
        }
    }
}

fn part1(program: &Program) -> i32 {
    let mut bootloader = BootLoader::new(program);
    match bootloader.boot() {
        Ok(_) => {
            panic!("bootloader finished unexpectedly");
        },
        Err(BootLoaderError::LoopDetected(_, acc)) => {
            acc
        },
        Err(e) => {
            panic!("{:?}", e);
        }
    }
}

fn part2(mut program: Program) -> i32 {
    let mut idx = 0;
    while idx < program.len() {
        if let Ok(_) = program.swap_instruction(idx) {
            let mut bootloader = BootLoader::new(&program);
            match bootloader.boot() {
                Ok(result) => {
                    return result;
                },
                Err(BootLoaderError::LoopDetected(_, _)) => {},
                Err(e) => {
                    panic!("{:?}", e);
                }
            };
            program.swap_instruction(idx).unwrap();
        }
        idx += 1;
    }
    panic!("Got to end of program with no match");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program: Program = parse_input_file("day_08_input.txt", parse_instruction)?;
    let result = part1(&program);
    println!("part1: {}", result);
    let result = part2(program);
    println!("part2: {}", result);
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    use adventofcode2020::parse_input_lines;

    #[test]
    fn test_1() -> Result<(), Box<dyn std::error::Error>> {
        let input = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;

        let program: Program = parse_input_lines(input.as_bytes(), parse_instruction)?;
        let result = part1(&program);
        assert_eq!(result, 5);

        let result = part2(program);
        assert_eq!(result, 8);

        Ok(())
    }
}
