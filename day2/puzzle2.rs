use serde_json::from_str;
use std::cmp;

fn split2<'a>(s: &'a str, splt: &str, msg: &str) -> (&'a str, &'a str) {
    let splt: Vec<&str> = s.split(splt).collect();
    match &splt[..] {
        [fst, snd] => (fst, snd),
        _ => panic!("{msg}")
    }
}

fn process_line(line: &str) -> u32 {
    let (gid_s, games) = split2(line, ": ", "failed to split line");
    let gid_splt: Vec<&str> = gid_s.split(" ").collect();
    let (_, id_s) = split2(gid_s, " ", "failed to split id");
    let id = from_str::<u32>(id_s).unwrap();
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for game in games.split("; ") {
        let rolls = game.split(", ");
        for roll in rolls {
            let (num_s, color) = split2(roll, " ", "failed to split id {id} roll {roll}");
            let num = from_str::<u32>(num_s).unwrap();
            if color == "red" {
                red = cmp::max(red, num);
            }
            if color == "green" {
                green = cmp::max(green, num);
            }
            if color == "blue" {
                blue = cmp::max(blue, num);
            }
        }
    }
    return red*green*blue;
}

fn puzzle2(input: &str) {
    let lines = input.split("\n");
    let mut sum = 0;
    for line in lines {
        sum += process_line(&line);
    }
    println!("{}", sum);
}
