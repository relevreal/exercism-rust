use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Debug>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (len_1, len_2) if len_1 == len_2 => {
            if first_list == second_list {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
        (len_1, len_2) if len_1 > len_2 => match compare(second_list, first_list) {
            Comparison::Sublist => Comparison::Superlist,
            c => c,
        },
        (_, _) => compare(first_list, second_list),
    }
}

fn compare<T: PartialEq + Debug>(first_list: &[T], second_list: &[T]) -> Comparison {
    let len_1 = first_list.len();
    let len_2 = second_list.len();
    for i in 0..=(len_2 - len_1) {
        if first_list == &second_list[i..(i + len_1)] {
            return Comparison::Sublist;
        }
    }
    Comparison::Unequal
}
