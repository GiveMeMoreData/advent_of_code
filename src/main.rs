use std::cmp::{max, min};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/days/3/input.txt")?;
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate(){
        let line_string = line.unwrap();
        let line_len = line_string.len();
        let (positions, values) = extract_numbers(line_string.clone());
        for (start, end) in positions{
            let surrounding = get_surrounding(start, end, i, line_len);
            for surrounding
            println!("{:?}", &line_string[start..end].parse::<i32>().unwrap())
        }
        // println!("{:?}", vals);
        println!("{:?}", line_string);
        break
    }

    Ok(())
}

fn extract_numbers(line: String) -> (Vec<(usize, usize)>, Vec<i32>){
    let mut idx: Vec<(usize, usize)> = Vec::new();
    let mut vals: Vec<(i32)> = Vec::new();
    let n = line.len();
    let mut number_start: usize = n;
    let chars = line.chars();

    for (i, char) in chars.enumerate(){
       if char.is_digit(10){
           if number_start == n{
               number_start = i;
               continue
           }

       } else {
           if number_start != n{
               idx.push((number_start, i);
               vals.push((line[number_start..i].parse::<i32>().unwrap()));
               number_start = n;
           }
       }
    }
    if number_start != n{
        idx.push((number_start, line.len()));
        vals.push((line[number_start..line.len()].parse::<i32>().unwrap()))
    }
    (idx, vals)
}

fn get_surrounding(start: usize, end: usize, line_nr: usize, max_line: usize) -> Vec<(usize, usize)>{
    let min_y = min(line_nr-1, 0);
    let max_y = max(line_nr+1, max_line);
    let mut surrounding = Vec::new();
    for x_idx in start..end{
        for y_idx in min_y..max_y{
            surrounding.push((x_idx, y_idx))
        }
    }
    surrounding
}

fn get_all_surrounding(positions: Vec<(usize, usize)>, line_nr: usize, max_line: usize) -> Vec<Vec<(usize, usize)>>{
    let mut idx = Vec::new();
    let min_y = min(line_nr-1, 0);
    let max_y = max(line_nr+1, max_line);
    for (start, end) in positions{
        let mut surrounding = Vec::new();
        for x_idx in start..end{
            for y_idx in min_y..max_y{
                surrounding.push((x_idx, y_idx))
            }
        }
        idx.push(surrounding);
    }
    idx
}
