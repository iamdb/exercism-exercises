#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.eq(second_list) {
        Comparison::Equal
    } else {
        if first_list.is_empty() {
            return Comparison::Sublist;
        } else if second_list.is_empty() {
            return Comparison::Superlist;
        } else if first_list.len() < second_list.len() {
            for window in second_list.windows(first_list.len()) {
                if window.eq(first_list) {
                    return Comparison::Sublist;
                }
            }
        } else {
            for window in first_list.windows(second_list.len()) {
                if window.eq(second_list) {
                    return Comparison::Superlist;
                }
            }
        }

        Comparison::Unequal
    }
}
