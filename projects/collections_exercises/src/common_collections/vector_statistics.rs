pub fn mean(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut count = 0;

    for i in v {
        sum = sum + i;
        count += 1;
    }

    let mean = sum / count;
    return mean;
}

use std::collections::HashMap;
pub fn mode(v: &Vec<i32>) -> Vec<i32> {
    let mut count_map = HashMap::new();

    for n in v {
        let count = count_map.entry(n).or_insert(0);
        *count += 1
    }

    let mut max_count = 0;
    let mut mode: Vec<i32> = Vec::new();
    for (number, count) in &count_map {
        if count > &max_count {
            mode = vec![**number];
            max_count = *count;
        } else if count == &max_count {
            if mode.contains(number) {
                continue;
            } else {
                mode.push(**number);
            }
        }
    }
    return mode;
}

pub fn median(v: &mut Vec<i32>) -> i32 {
    let mut median = 0;
    v.sort();
    if v.len() % 2 == 0 {
        let half = v.len() / 2;
        median = (v[half] + v[half + 1]) / 2;
    } else if v.len() % 2 == 1 {
        let lower_half = v.len() - 1;
        median = v[(lower_half / 2)];
    }
    return median;
}
