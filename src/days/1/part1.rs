use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/days/1/input.txt")?;
    let reader = BufReader::new(file);


    let mut codes_sum: i32 = 0;
    for line in reader.lines() {
        let vals = extract_ints_from_line(line.unwrap());
        if vals.is_empty() {continue}
        let code = extract_code_from_ints(vals);
        codes_sum += i32::from(code);
    }
    println!("{}", codes_sum);

    Ok(())
}

fn extract_ints_from_line(line: String) -> Vec<i8>{
    let mut ints: Vec<i8> = Vec::new();
    for char in line.chars(){
        if char.is_digit(10) {
            ints.push(char.to_string().parse().unwrap())
        }
    }
    ints
}

fn extract_code_from_ints(values: Vec<i8>) -> i8{
    let len = values.len();
    if len == 1{
        return values[0]*10 + values[0];
    }
    return values[0]*10 + values[len-1];
}
