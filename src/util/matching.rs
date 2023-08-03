pub fn find_matching_characters(
    full_list: &Vec<char>,
    char_start: char,
    char_end: char,
    offset: usize,
) -> Option<usize> {
    let mut balance = 0;
    let iterator = offset..(full_list.len());
    for c in iterator {
        if full_list[c] == char_start {
            balance += 1;
        } else if full_list[c] == char_end {
            balance -= 1;
        }

        if balance == 0 {
            return Some(c);
        }
    }
    return None;
}

pub fn find_matching_characters_reversed(
    full_list: &Vec<char>,
    char_start: char,
    char_end: char,
    offset: usize,
) -> Option<usize> {
    let mut balance = 0;
    let iterator = (0..(offset + 1)).rev();
    for c in iterator {
        if full_list[c] == char_start {
            balance += 1;
        } else if full_list[c] == char_end {
            balance -= 1;
        }

        if balance == 0 {
            return Some(c);
        }
    }
    return None;
}

pub fn find_matching_substring(
    full_list: &Vec<char>,
    substr_start: &str,
    substr_end: &str,
    offset: usize,
) -> Option<usize> {
    let mut balance = 0;
    let iterator = offset..(full_list.len());

    for index in iterator {
        if !(index % 2 == 0) {
            continue;
        }
        if format!("{}{}", full_list[index], full_list[index + 1]) == substr_start {
            balance += 1;
        } else if format!("{}{}", full_list[index], full_list[index + 1]) == substr_end {
            balance -= 1;
        }

        if balance == 0 {
            return Some(index);
        }
    }
    return None;
}

pub fn find_matching_substring_reversed(
    full_list: &Vec<char>,
    substr_start: &str,
    substr_end: &str,
    offset: usize,
) -> Option<usize> {
    let mut balance = 0;
    let iterator = (0..(offset + 1)).rev();

    for index in iterator {
        if !(index % substr_start.len() == 0) {
            continue;
        }
        if format!("{}{}", full_list[index], full_list[index + 1]) == substr_start {
            balance += 1;
        } else if format!("{}{}", full_list[index], full_list[index + 1]) == substr_end {
            balance -= 1;
        }

        if balance == 0 {
            return Some(index);
        }
    }
    return None;
}
