const INPUT_FILE: &str = "input.txt";

fn main() {
    println!(
        "{}",
        std::fs::read_to_string(INPUT_FILE)
            .expect("Error opening the input file")
            .split('\n')
            .map(|s| s.split(' '))
            .map(|mut v| {
                (
                    v.next().expect("Expected the command first"),
                    v.next()
                        .expect("Expected the amount second")
                        .parse::<u32>()
                        .expect("Count not parse u32 from input"),
                )
            })
            .fold(
                (0, 0, 0),
                |(hor, depth, _), (command, amount)| match command {
                    "forward" => (hor + amount, depth, (hor + amount) * depth),
                    "up" => (hor, depth - amount, hor * (depth - amount)),
                    "down" => (hor, depth + amount, hor * (depth + amount)),
                    _ => panic!("unknown command {}", command),
                },
            )
            .2
    );
}
