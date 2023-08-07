use std::fmt::Display;

pub enum IteratorOrder {
    FrontToBack,
    BackToFront,
}

pub fn find_matching<T: Sized + PartialEq>(
    full_list: &impl AsRef<[T]>,
    loop_start: T,
    loop_end: T,
    offset: usize,
    order: IteratorOrder,
) -> Option<usize> {
    let mut balance = 0;

    let main_iterator: Box<dyn Iterator<Item = usize>> = {
        match order {
            IteratorOrder::FrontToBack => Box::new(offset..(full_list.as_ref().len())),
            IteratorOrder::BackToFront => Box::new((0..offset + 1).rev()),
        }
    };

    for single_obj in main_iterator {
        let mogu = &full_list.as_ref()[single_obj];
        if *mogu == loop_start {
            balance += 1;
        } else if *mogu == loop_end {
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
    order: IteratorOrder,
) -> Option<usize> {
    let mut balance = 0;

    let main_iterator: Box<dyn Iterator<Item = usize>> = {
        match order {
            IteratorOrder::FrontToBack => Box::new(offset..(full_list.as_ref().len())),
            IteratorOrder::BackToFront => Box::new((0..offset + 1).rev()),
        }
    };

    for index in main_iterator {
        if !(index % substr_start.len() == 0) {
            continue;
        }

        let mogu = format!(
            "{}{}",
            full_list.as_ref().get(index).expect("Failed to read character in list"),
            full_list
                .as_ref()
                .get(index + 1)
                .expect("Failed to read character in list")
        );

        if mogu == substr_start {
            balance += 1;
        } else if mogu == substr_end {
            balance -= 1;
        }

        if balance == 0 {
            return Some(index);
        }
    }
    return None;
}
