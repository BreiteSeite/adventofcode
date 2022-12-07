use std::fs::read_to_string;

pub fn day1() {
    let input = read_to_string("./input/day1_calorie_counting").expect("could not read input file");

    let chunks = input.split("\n\n");

    let mut chunks_sum: Vec<u32> = Vec::new();

    for chunk in chunks {
        let chunk_sum: u32 = chunk
            .lines()
            .map(|s| s.parse::<u32>().expect("could not parse calorie value"))
            .sum();

        chunks_sum.push(chunk_sum);
    }

    chunks_sum.sort_unstable();

    println!(
        "Most total calories: {}",
        chunks_sum.iter().rev().take(1).sum::<u32>()
    );

    println!(
        "Most total calories (top 3): {}",
        chunks_sum.iter().rev().take(3).sum::<u32>()
    );
}
