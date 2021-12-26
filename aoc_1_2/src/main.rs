const INPUT_FILE: &str = "input.txt";

fn main() {
    let v = std::fs::read_to_string(INPUT_FILE)
        .expect("Error opening the input file")
        .split('\n')
        .map(|s| s.parse::<u32>().expect("could not parse u32 from input"))
        .collect::<Vec<u32>>();

    println!(
        "{}",
        v.iter()
            .zip(v.iter().skip(1))
            .zip(v.iter().skip(2))
            .map(|((a, b), c)| a + b + c)
            .fold((0, None), |(sum, prev), i| match prev {
                Some(x) if x < i => (sum + 1, Some(i)),
                Some(_) | None => (sum, Some(i)),
            })
            .0
    );
}
