const INPUT_FILE: &str = "input.txt";

fn main() {
    println!(
        "{}",
        std::fs::read_to_string(INPUT_FILE)
            .expect("Error opening the input file")
            .split('\n')
            .map(|s| s.parse::<u32>().expect("could not parse u32 from input"))
            .fold((0, None), |(sum, prev), i| match prev {
                Some(x) if x < i => (sum + 1, Some(i)),
                Some(_) | None => (sum, Some(i)),
            })
            .0
    );
}
