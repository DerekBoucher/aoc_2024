struct Report {
    levels: Vec<i32>,
}

fn main() {
    let env = env_logger::Env::default().filter_or("RUST_LOG", "debug");
    env_logger::init_from_env(env);
    log::info!("Advent of code 2024!!!");
    log::info!("-----------------------------");
    log::info!("Day 2: Red-Nosed Reports");

    let input = std::fs::read_to_string("day_2/input.txt").unwrap();
    let seperate_lines: Vec<String> = input.lines().map(|x| x.to_string()).collect();

    let mut reports: Vec<Report> = vec![];

    for line in seperate_lines {
        let mut levels: Vec<i32> = vec![];

        line.split(" ").for_each(|str| {
            levels.push(str.parse::<i32>().unwrap());
        });

        reports.push(Report { levels });
    }

    let mut count_safe = 0;

    for report in reports {
        let mut last_level = 0;
        let mut is_increasing = false;
        for (i, current_level) in report.levels.iter().enumerate() {
            if i == 0 {
                last_level = *current_level;
                continue;
            }

            if i == 1 {
                if !is_within(last_level, *current_level, 1, 3) {
                    // not safe
                    break;
                }

                if *current_level > last_level {
                    is_increasing = true;
                }
                last_level = *current_level;
                continue;
            }

            if is_increasing {
                if *current_level < last_level {
                    // not safe
                    break;
                }
            } else {
                if *current_level > last_level {
                    // not safe
                    break;
                }
            }

            if !is_within(last_level, *current_level, 1, 3) {
                // not safe
                break;
            }

            last_level = *current_level;

            if i == report.levels.len() - 1 {
                count_safe += 1;
            }
        }
    }

    log::info!("Count of safe reports: {}", count_safe);
}

fn is_within(current_level: i32, other_level: i32, low_bound: i32, high_bound: i32) -> bool {
    let delta = (current_level - other_level).abs();

    if delta < low_bound {
        return false;
    }

    if delta > high_bound {
        return false;
    }

    return true;
}
