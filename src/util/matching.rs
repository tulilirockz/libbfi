use std::fmt::Display;

pub fn find_matching<T: Sized + Eq>(
    full_list: &impl AsRef<[T]>,
    loop_start: T,
    loop_end: T,
    offset: usize,
    reversed_order: bool,
) -> Option<usize> {
    let mut balance = 0;

    let main_iterator: Box<dyn Iterator<Item = usize>>;

    if !reversed_order {
        main_iterator = Box::new(offset..(full_list.as_ref().len()));
    } else {
        main_iterator = Box::new((0..offset + 1).rev());
    }

    for single_obj in main_iterator {
        if full_list.as_ref()[single_obj] == loop_start {
            balance += 1;
        } else if full_list.as_ref()[single_obj] == loop_end {
            balance -= 1;
        }

        if balance == 0 {
            return Some(single_obj);
        }
    }
    return None;
}

pub fn find_matching_substring<T: Eq + Sized + Display>(
    full_list: &impl AsRef<[T]>,
    substr_start: &str,
    substr_end: &str,
    offset: usize,
    reversed_order: bool,
) -> Option<usize> {
    let mut balance = 0;
    let main_iterator: Box<dyn Iterator<Item = usize>>;

    if !reversed_order {
        main_iterator = Box::new(offset..(full_list.as_ref().len()));
    } else {
        main_iterator = Box::new((0..offset + 1).rev());
    }

    for index in main_iterator {
        if !(index % substr_start.len() == 0) {
            continue;
        }

        let full_obj = {
            format!(
                "{}{}",
                full_list.as_ref()[index],
                full_list.as_ref()[index + 1]
            )
        };

        if full_obj == substr_start {
            balance += 1;
        } else if full_obj == substr_end {
            balance -= 1;
        }

        if balance == 0 {
            return Some(index);
        }
    }
    return None;
}
