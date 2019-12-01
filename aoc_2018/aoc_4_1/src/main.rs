use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("input.txt").lines().collect::<Vec<_>>();
    lines.sort();
    let lines = lines.iter().map(|line| {
        line.split(|c| c == ' ' || c == ':' || c == '#' || c == ']')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<_>>()
    });
    let mut current_guard = 0;
    let mut guard_asleep = false;
    let mut asleep_time = 0;
    let mut asleep_counter = HashMap::new();

    for line in lines {
        if line.len() == 3 {
            current_guard = line[2];
            continue;
        }

        if guard_asleep {
            let time_slept = (asleep_time, line[1]);
            asleep_counter
                .entry(current_guard)
                .or_insert(vec![])
                .push(time_slept);
        } else {
            asleep_time = line[1];
        }
        guard_asleep = !guard_asleep;
    }

    let mut max_sleep_minute = 0;
    let mut max_sleep_minute_time = 0;

    let mut times = asleep_counter
        .iter()
        .map(|(guard, schedule)| {
            (
                schedule.iter().map(|t| t.1 - t.0).sum::<i32>(),
                guard,
                schedule,
            )
        })
        .collect::<Vec<_>>();
    times.sort();
    let sleepy_guard = times.get(times.len() - 1).unwrap().1;

    println!("Sleepy Guard: {}", sleepy_guard);

    let mut minutes: [i32; 60] = [0; 60];

    for interval in times
        .iter()
        .filter(|x| x.1 == sleepy_guard)
        .next()
        .unwrap()
        .2
    {
        for m in interval.0..interval.1 {
            let m = m as usize;
            minutes[m] += 1;
            if minutes[m] > max_sleep_minute_time {
                max_sleep_minute_time = minutes[m];
                max_sleep_minute = m as i32;
            }
        }
    }

    println!(
        "{} * {} -> {}",
        sleepy_guard,
        max_sleep_minute,
        sleepy_guard * max_sleep_minute
    );
}
