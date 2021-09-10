use std::env::{args, var};
use std::io::{stdin, Bytes, Read, StdinLock};

fn main() {
    let file = var("PWD").unwrap() + "/" + &args().nth(1).expect("expected a filename");

    let file = std::fs::read(file).unwrap();

    let input = stdin();

    let mut interpreter = Interpreter::new(file, input.lock().bytes());

    interpreter.run();
}

struct Interpreter<'a> {
    mem: [u8; 30_000],
    pc: usize,
    stack: Vec<usize>,
    pointer: usize,
    file: Vec<u8>,
    input: Bytes<StdinLock<'a>>,
}

impl<'a> Interpreter<'a> {
    fn new(file: Vec<u8>, input: Bytes<StdinLock<'a>>) -> Self {
        Self {
            mem: [0; 30_000],
            pc: 0,
            stack: Vec::new(),
            pointer: 0,
            file,
            input,
        }
    }

    fn run(&mut self) {
        while self.pc < self.file.len() {
            self.execute();
        }
    }

    fn execute(&mut self) {
        let c = self.fetch();

        match c {
            '+' => self.mem[self.pointer] += 1,
            '-' => self.mem[self.pointer] -= 1,
            '<' => self.pointer -= 1,
            '>' => self.pointer += 1,
            '[' => {
                if self.mem[self.pointer] == 0 {
                    self.pc += 1;
                    let mut count: i32 = 0;
                    while self.pc < self.file.len() && (self.fetch() != ']' || count > 0) {
                        match self.fetch() {
                            '[' => count += 1,
                            ']' => count -= 1,
                            _ => {}
                        }

                        self.pc += 1;
                    }
                } else {
                    self.stack.push(self.pc);
                }
            }

            ']' => {
                if self.mem[self.pointer] != 0 {
                    self.pc = *self.stack.last().unwrap();
                } else {
                    self.stack.pop();
                }
            }

            ',' => self.mem[self.pointer] = self.input.next().unwrap().unwrap(),
            '.' => print!("{}", self.mem[self.pointer] as char),
            _ => {}
        }

        self.pc += 1;
    }

    fn fetch(&self) -> char {
        self.file[self.pc] as char
    }
}
