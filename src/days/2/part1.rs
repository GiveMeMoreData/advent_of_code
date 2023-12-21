use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/days/2/input.txt")?;
    let reader = BufReader::new(file);

    let game_limits: HashMap<String, i8> = HashMap::from([
        ("red".to_string(),12),
        ("green".to_string(),13),
        ("blue".to_string(),14),
    ]);

    let mut codes_sum: i32 = 0;
    for line in reader.lines(){
        let x = line.unwrap();
        let vals = x.splitn(2, ":").collect::<Vec<&str>>();
        let game_id = extract_game_id(vals[0].to_string());
        let decoded = process_game_log(&vals[1]);

        if game_valid(&game_limits, decoded){
            codes_sum += game_id;
        }
    }
    println!("{}", codes_sum);

    Ok(())
}


fn game_valid(limits: &HashMap<String, i8>, game:HashMap<String, i8>)  -> bool{
    for (key,val) in game{
        if val > *limits.get(key.as_str()).unwrap(){
            return false
        }
    }
    true
}

fn extract_game_id(game_name: String) -> i32{
    let (_, id_str) = game_name.rsplit_once(" ").unwrap();
    id_str.parse::<i32>().unwrap()
}

fn process_game_log(game_log: &str) -> HashMap<String, i8> {

    let mut log: HashMap<String, i8> = HashMap::from([
        ("red".to_string(),0),
        ("green".to_string(),0),
        ("blue".to_string(),0),
    ]);

    let game_sets = game_log.split(";");
    for set in game_sets{
        let records = set.split(",");
        for record in records{
            let vals = record.split_whitespace().collect::<Vec<&str>>();
            let key = vals[1].to_string();
            let amount = vals[0].parse::<i8>().unwrap();
            if amount > *log.get(key.as_str()).unwrap(){
                log.insert(key, amount);
            }
        }
    }
    log
}

