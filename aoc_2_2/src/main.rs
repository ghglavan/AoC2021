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
                (0, 0, 0, 0),
                |(hor, depth, aim, _), (command, amount)| match command {
                    "forward" => (
                        hor + amount,
                        depth + (aim * amount),
                        aim,
                        (hor + amount) * (depth + (aim * amount))
                    ),
                    "up" => (hor, depth, aim - amount, hor * depth),
                    "down" => (hor, depth, aim + amount, hor * depth),
                    _ => panic!("unknown command {}", command),
                },
            )
            .3
    );
}
