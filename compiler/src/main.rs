use std::env::{args, var};
use std::fs::{read_to_string, write};
use std::process::Command;

fn main() {
    let file = var("PWD").unwrap() + "/" + &args().nth(1).expect("expected a filename");

    let contents = read_to_string(file).unwrap();

    let transpiled: Vec<String> = contents
        .chars()
        .fold(Vec::new(), compress)
        .into_iter()
        .map(transpile)
        .collect();

    let flattened: String = transpiled.iter().map(|x| x.chars()).flatten().collect();

    let rs_code: String = String::from(
        r#"
    #![allow(unused)]

    use std::io::{ Read, stdin, stdout, Write };

    fn main() {
         let mut mem = [0_u8; 30_000];
         let mut pointer = 0;
         let input = stdin();
         let mut input = input.lock().bytes();
    "#,
    ) + &flattened
        + "}";

    write(var("HOME").unwrap() + "/_rust.rs", rs_code).unwrap();

    Command::new("rustc")
        .args(&["-O", "_rust.rs"])
        .current_dir(var("HOME").unwrap())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    Command::new(var("HOME").unwrap() + "/_rust")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn compress(mut res: Vec<(usize, u16)>, instr: char) -> Vec<(usize, u16)> {
    let valid = ['+', '-', ',', '.', '<', '>', '[', ']'];

    if let Some((x, count)) = res.last_mut() {
        if instr == valid[*x] && instr != '[' && instr != ']' {
            *count += 1;
            return res;
        }
    }

    if let Some(idx) = valid.iter().position(|x| *x == instr) {
        res.push((idx, 1));
    }

    res
}

fn transpile((code, count): (usize, u16)) -> String {
    let rust = [
        ("mem[pointer] += ", ";\n"),
        ("mem[pointer] -= ", ";\n"),
        ("mem[pointer] = input.nth(", " - 1).unwrap().unwrap();\n"),
        (
            "for _ in 0..",
            " { print!(\"{}\", mem[pointer] as char); } stdout().flush().unwrap();\n",
        ),
        ("pointer -= ", ";\n"),
        ("pointer += ", ";\n"),
        ("while mem[pointer] > 0 {\n", ""),
        ("}\n", ""),
    ];

    if code < 6 {
        format!("{}{}{}", rust[code].0, count, rust[code].1)
    } else {
        rust[code].0.to_string().repeat(count.into())
    }
}
