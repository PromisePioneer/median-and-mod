use log::log;
use std::collections::HashMap;

fn main() {
    let rounded_value = vec![1, 2, 4, 5, 5, 5, 6, 6];
    let middle_value = get_middle_value(&rounded_value);
    println!("The Middle value is: {middle_value:?}");

    let mode = get_mode_value(rounded_value);

    println!("The mode value is : {mode:?}");

    let mut str1 = String::from("first");
}





fn get_mode_value(value: Vec<u32>) -> Option<u32> {
    let mut map = HashMap::new();
    for v in value {
        *map.entry(v).or_insert(0) += 1;
    }
    let mut max_count = 0;
    let mut mode_value = None;

    for (key, count) in map {
        if count > max_count {
            max_count = count;
            mode_value = Some(key);
        }
    }

    mode_value
}

fn get_middle_value(v: &Vec<u32>) -> Option<u32> {
    if v.is_empty() {
        return None;
    }

    let mid_index = v.len() / 2;
    v.get(mid_index).copied()
}
