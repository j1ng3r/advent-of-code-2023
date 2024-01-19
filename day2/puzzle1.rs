use serde_json::from_str;

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
    for game in games.split("; ") {
        let rolls = game.split(", ");
        for roll in rolls {
            let (num_s, color) = split2(roll, " ", "failed to split id {id} roll {roll}");
            let num = from_str::<u32>(num_s).unwrap();
            if color == "red" && num > 12 {
                return 0;
            }
            if color == "green" && num > 13 {
                return 0;
            }
            if color == "blue" && num > 14 {
                return 0;
            }
            
        }
    }
    return id;
}

fn puzzle1(input: &str) {
    let lines = input.split("\n");
    let mut sum = 0;
    for line in lines {
        sum += process_line(&line);
    }
    println!("{}", sum);
}
