use std::env::{args, var};
use std::io::{stdin, Bytes, Read, StdinLock};

fn main() {
    let file = var("PWD").unwrap() + "/" + &args().nth(1).expect("expected a filename");

    let file = std::fs::read(file).unwrap();

    let input = stdin();

    let mut interpreter = Interpreter::new(file, input.lock().bytes());

    interpreter.run();
}

macro_rules! repeat {
    ($times: ident, $s: stmt) => {{
        for _ in 0..$times {
            $s
        }
    }};
}

struct Interpreter<'a> {
    mem: [u8; 30_000],
    pc: usize,
    stack: Vec<usize>,
    pointer: usize,
    program: Vec<(u8, u16)>,
    input: Bytes<StdinLock<'a>>,
}

impl<'a> Interpreter<'a> {
    fn new(file: Vec<u8>, input: Bytes<StdinLock<'a>>) -> Self {
        Self {
            mem: [0; 30_000],
            pc: 0,
            stack: Vec::new(),
            pointer: 0,
            program: Self::preprocess(file),
            input,
        }
    }

    fn run(&mut self) {
        while self.pc < self.program.len() {
            self.execute();
        }
    }

    fn execute(&mut self) {
        let (c, count) = self.fetch();

        match c {
            '+' => self.mem[self.pointer] = self.mem[self.pointer].wrapping_add(count as u8),
            '-' => self.mem[self.pointer] = self.mem[self.pointer].wrapping_sub(count as u8),
            '<' => self.pointer -= usize::from(count),
            '>' => self.pointer += usize::from(count),
            ',' => repeat!(
                count,
                self.mem[self.pointer] = self.input.next().unwrap().unwrap()
            ),
            '.' => repeat!(count, print!("{}", self.mem[self.pointer] as char)),

            '[' => {
                if self.mem[self.pointer] == 0 {
                    self.pc += 1;
                    let mut count: u32 = 0;

                    while self.pc < self.program.len() && (self.fetch().0 != ']' || count > 0) {
                        match self.fetch() {
                            ('[', x) => count = count.checked_add(x as u32).unwrap(),
                            (']', x) => count = count.checked_sub(x as u32).unwrap(),
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

            _ => {}
        }

        self.pc += 1;
    }

    fn fetch(&self) -> (char, u16) {
        (self.program[self.pc].0 as char, self.program[self.pc].1)
    }

    fn preprocess(file: Vec<u8>) -> Vec<(u8, u16)> {
        let valid = ['+', '-', '[', ']', ',', '.', '<', '>'];

        let mut res = Vec::with_capacity(file.len());

        for i in file {
            if let Some((x, count)) = res.last_mut() {
                if i == *x && i as char != '[' && i as char != ']' {
                    *count += 1;
                    continue;
                }
            }

            if valid.contains(&(i as char)) {
                res.push((i, 1));
            }
        }

        res
    }
}
