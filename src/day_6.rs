pub fn values() -> &'static str {
    "Time:        44     80     65     72
    Distance:   208   1581   1050   1102"
}
pub fn test_values() -> &'static str{
    "Time:      7  15   30
    Distance:  9  40  200"
}
pub fn p1(values: &str) -> u64 {
    let t_n_d = values.split("\n").collect::<Vec<&str>>();
    let (mut times, mut distances) = (t_n_d[0], t_n_d[1]);
    let mut times_vec = times.split(":").collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>();
    times_vec.retain(|value| !value.is_empty());
    let mut distances_vec = distances.split(":").collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>();
    distances_vec.retain(|value| !value.is_empty());
    let mut t_n_d_tuples_vec: Vec<(u64, u64)> = vec![];
    for i in 0..times_vec.len() {
        t_n_d_tuples_vec.push((
            times_vec[i].parse::<u64>().unwrap(),
            distances_vec[i].parse::<u64>().unwrap(),
        ))
    }
    let mut ways_multiplied = 1;
    for time_distance in t_n_d_tuples_vec {
        let mut ways_to_beat_ind_distance = 0;
        for i in 0..time_distance.0 + 1 {
            if i * (time_distance.0 - i) > time_distance.1 {
                ways_to_beat_ind_distance += 1;
            }
        }
        println!("Time {} and Distance {} has {} ways to beat", time_distance.0, time_distance.1, ways_to_beat_ind_distance);
        ways_multiplied *= ways_to_beat_ind_distance;
    }
    ways_multiplied
}
pub fn p2(values: &str) -> u64 {
    let t_n_d = values.split("\n").collect::<Vec<&str>>();
    let (mut time, mut distance) = (t_n_d[0], t_n_d[1]);
    let mut time_vec = time.split(":").collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>();
    time_vec.retain(|value| !value.is_empty());
    let mut full_time = String::from("");
    for c in time_vec{
        full_time.push_str(c);
    }
    let mut distances_vec = distance.split(":").collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>();
    distances_vec.retain(|value| !value.is_empty());
    let mut full_distance = String::from("");
    for c in distances_vec{
        full_distance.push_str(c)
    }
    let mut ways_to_beat_ind_distance = 0;
    // I bet you could cut the time in half and just +2 for each pass, +1 at the end if it's an odd number
    for i in 0..full_time.parse::<u64>().unwrap() + 1 {
        if i * (full_time.parse::<u64>().unwrap() - i) > full_distance.parse::<u64>().unwrap() {
            ways_to_beat_ind_distance += 1;
        }
    }
    println!("Time {} and Distance {} has {} ways to beat", full_time, full_distance, ways_to_beat_ind_distance);
    ways_to_beat_ind_distance
}
