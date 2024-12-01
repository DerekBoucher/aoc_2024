fn main() {
    let env = env_logger::Env::default().filter_or("RUST_LOG", "debug");
    env_logger::init_from_env(env);
    log::info!("Advent of code 2024!!!");
    log::info!("-----------------------------");
    log::info!("Day 1: Historian Hysteria");

    let input = std::fs::read_to_string("day_1/input.txt").unwrap();
    let seperate_lines: Vec<String> = input.lines().map(|x| x.to_string()).collect();
    let mut left_location_ids: Vec<i32> = vec![];
    let mut right_location_ids: Vec<i32> = vec![];

    for line in seperate_lines {
        log::debug!("Line: {}", line);
        let vals = line.split("   ").collect::<Vec<&str>>();
        let left = vals[0].parse::<i32>().unwrap();
        let right = vals[1].parse::<i32>().unwrap();
        left_location_ids.push(left);
        right_location_ids.push(right);
    }

    left_location_ids.sort();
    right_location_ids.sort();

    let mut sum_of_differences: i32 = 0;
    left_location_ids
        .iter()
        .zip(right_location_ids.iter())
        .for_each(|(left, right)| {
            let difference = right - left;
            sum_of_differences += difference.abs();
        });

    log::info!("-----------------------------");
    log::info!("Part 1, Sum of differences: {}", sum_of_differences);

    let mut similarity_score = 0;

    left_location_ids.iter().for_each(|left| {
        let mut occurrences = 0;

        right_location_ids.iter().for_each(|right| {
            if left == right {
                occurrences += 1;
            }
        });

        similarity_score += left * occurrences;
    });

    log::info!("Part 2, Similarity score: {}", similarity_score);
}
