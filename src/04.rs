fn main() -> std::io::Result<()> {
    let start = 246540;
    //let end = 247000;
    let end = 787419;

    let numbers: Vec<i32> = (start..=end).collect();

    let mut valid_passwords = 0;
    for number in numbers {
        let mut duplicate = false;
        let mut increasing = true;
        let chars = number.to_string().into_bytes();
        for i in chars.windows(2) {
            if i[1] < i[0] {
                increasing = false;
            }
            if i[0] == i[1] {
                duplicate = true;
            }
        }
        if increasing && duplicate {
            valid_passwords += 1;
        }
    }
    println!("valid_passwords: {}", valid_passwords);

    Ok(())
}
