pub fn main() {
    let mut buffer = String::with_capacity(20);
    std::io::stdin().read_line(&mut buffer).expect("Failed to read line");

    match buffer.trim().parse::<u32>() {
        Ok(weight) if weight % 2 == 0 && weight > 3 => println!("YES"),
        _ => {
            println!("NO")
        }
    }
}
