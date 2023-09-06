use std::env::{args, var};
use std::fs::{self, read_to_string, write};
use std::process::Command;

fn main() {
    let path =
        fs::canonicalize(args().nth(1).expect("expected a filename")).expect("invalid file name");
    let mut rs_src_path = path.clone();
    rs_src_path.set_file_name("_rust.rs");
    let mut rs_dst_path = path.clone();
    rs_dst_path.set_file_name("_rust");

    let contents = read_to_string(path).unwrap();

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

    write(rs_src_path.as_path(), rs_code).unwrap();

    Command::new("rustc")
        .args(&["-C", "opt-level=3", rs_src_path.to_str().unwrap()])
        .current_dir(rs_src_path.parent().unwrap())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    Command::new(rs_dst_path.to_str().unwrap())
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
