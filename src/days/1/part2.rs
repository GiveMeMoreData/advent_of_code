use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/days/1/input.txt")?;
    let reader = BufReader::new(file);

    let digits_map: HashMap<&str, i8> = HashMap::from([
        ("one",1),
        ("two",2),
        ("three",3),
        ("four",4),
        ("five",5),
        ("six",6),
        ("seven",7),
        ("eight",8),
        ("nine",9),
    ]);

    let mut codes_sum: i32 = 0;
    for line in reader.lines() {
        let vals = extract_str_digits(line.unwrap(), &digits_map);
        if vals.is_empty() {continue}
        let code = extract_code_from_ints(vals);
        codes_sum += i32::from(code);
    }
    println!("{}", codes_sum);

    Ok(())
}

fn extract_str_digits(line: String, str_map: &HashMap<&str, i8>) -> Vec<i8> {

    let mut ints: Vec<i8> = Vec::new();
    let n = line.len();

    for (i, char) in line.chars().enumerate(){
        if char.is_digit(10) {
            ints.push(char.to_string().parse().unwrap());
            continue
        }
        for j in 3..6{
            if n < i + j {break}
            match str_map.get(&line[i..i+j]) {
                Some(digit) => ints.push(*digit),
                None => {}
            }
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
