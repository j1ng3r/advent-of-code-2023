fn puzzle1(input: &str) {
    let lines = input.split("\n");
    let mut sum = 0;
    for line in lines {
        let mut digit1: u32 = 10;
        let mut digit2: u32 = 0;
        for chr in line.chars() {
            match chr.to_digit(10) {
                Some(digit) => {
                    digit2 = digit;
                    if digit1 == 10 {
                        digit1 = digit;
                    }
                }
                None => {}
            }
        }
        sum += 10 * digit1 + digit2;
    }
    println!("{}", sum);
}
