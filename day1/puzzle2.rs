fn find_ends<'a>(line: &str, sub: &'a str) -> Option<(usize, usize, &'a str)> {
    let mut sub_i = 0;
    let mut hits = None;
    for (i, chr) in line.chars().enumerate() {
        if Some(chr) == sub.chars().nth(sub_i) {
            if sub_i == sub.len() - 1 {
                let hit = i - sub_i;
                hits = match hits {
                    Some((fst, _lst, _sub)) => Some((fst, hit, sub)),
                    None => Some((hit, hit, sub))
                };
                sub_i = 0;
            } else {
                sub_i += 1;
            }
        } else if Some(chr) == sub.chars().nth(0) {
            if 1 == sub.len() {
                let hit = i;
                hits = match hits {
                    Some((fst, _lst, _sub)) => Some((fst, hit, sub)),
                    None => Some((hit, hit, sub))
                };
                sub_i = 0;
            } else {
                sub_i = 1;
            }
        } else {
            sub_i = 0;
        }
    }
    hits
}

fn puzzle2(input: &str) {
    let lines = input.split("\n");
    let mut sum = 0;
    for line in lines {
        let v = vec![
            (find_ends(line, "1"), 1),
            (find_ends(line, "2"), 2),
            (find_ends(line, "3"), 3),
            (find_ends(line, "4"), 4),
            (find_ends(line, "5"), 5),
            (find_ends(line, "6"), 6),
            (find_ends(line, "7"), 7),
            (find_ends(line, "8"), 8),
            (find_ends(line, "9"), 9),
            (find_ends(line, "0"), 0),
            (find_ends(line, "one"), 1),
            (find_ends(line, "two"), 2),
            (find_ends(line, "three"), 3),
            (find_ends(line, "four"), 4),
            (find_ends(line, "five"), 5),
            (find_ends(line, "six"), 6),
            (find_ends(line, "seven"), 7),
            (find_ends(line, "eight"), 8),
            (find_ends(line, "nine"), 9),
            (find_ends(line, "zero"), 0)
        ];
        let mut min = None;
        let mut max = None;
        for (hits, digit) in v {
            match hits {
                Some((fst, lst, sub)) => {
                    match min {
                        Some((min_idx, _min_digit, _sub)) => {
                            if fst < min_idx {
                                min = Some((fst, digit, sub));
                            }
                        }
                        None => {
                            min = Some((fst, digit, sub));
                        }
                    }
                    match max {
                        Some((max_idx, _max_digit, _sub)) => {
                            if lst > max_idx {
                                max = Some((lst, digit, sub));
                            }
                        }
                        None => {
                            max = Some((lst, digit, sub));
                        }
                    }
                }
                None => {}
            }
        }
        match min {
            Some((min_idx, min_digit, min_sub)) => {
                match max {
                    Some((max_idx, max_digit, max_sub)) => {
                        let prefix = &line[0..min_idx];
                        let fst = &line[min_idx..min_idx+min_sub.len()];
                        let lst = &line[max_idx..max_idx+max_sub.len()];
                        let postfix = &line[max_idx+max_sub.len()..];
                        println!("{min_digit}{max_digit}, {prefix}#{fst}#{lst}#{postfix}");
                        sum += min_digit * 10 + max_digit;
                    }
                    None => {
                        println!("failed to get max");
                    }
                }
            }
            None => {
                println!("failed to get min");
            }
        }
    }
    println!("{}", sum);
}
