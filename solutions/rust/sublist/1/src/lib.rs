#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    if is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    }

    if is_superlist(first_list, second_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}

fn is_superlist(first: &[i32], second: &[i32]) -> bool {
    second.is_empty() || first.windows(second.len()).any(|window| window == second)
}

fn is_sublist(first: &[i32], second: &[i32]) -> bool {
    first.is_empty() || second.windows(first.len()).any(|window| window == first)
}
