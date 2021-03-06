#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + std::fmt::Debug + std::clone::Clone + Copy + Ord>(
    _first_list: &[T],
    _second_list: &[T],
) -> Comparison {
    let original_first = _first_list;
    let original_second = _second_list;

    let mut _first_list = _first_list.to_vec();
    let mut _second_list = _second_list.to_vec();

    _first_list.dedup();
    _second_list.dedup();

    if _first_list.len() == 0 && _first_list.len() < _second_list.len() {
        return Comparison::Sublist;
    } else if _second_list.len() == 0 && _first_list.len() > _second_list.len() {
        return Comparison::Superlist;
    }

    let mut last_index: usize;
    let mut start_index: usize;

    if _first_list.len() < _second_list.len() {
        loop {
            start_index = match _second_list.iter().position(|i| i == &_first_list[0]) {
                Some(v) => v,
                None => {
                    return Comparison::Unequal;
                }
            };

            last_index = if start_index + _first_list.len() > _second_list.len() {
                _second_list.len()
            } else {
                start_index + _first_list.len()
            };

            if &_second_list[last_index - 1] == _first_list.last().unwrap() {
                break;
            } else {
                _second_list.remove(start_index);
            }
        }

        let assert_to_list = &_second_list[start_index..last_index].to_vec();

        if _first_list == *assert_to_list {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        }
    } else if _first_list.len() > _second_list.len() {
        loop {
            start_index = match _first_list.iter().position(|i| i == &_second_list[0]) {
                Some(v) => v,
                None => return Comparison::Unequal,
            };

            last_index = if start_index + _second_list.len() > _first_list.len() {
                _first_list.len()
            } else {
                start_index + _second_list.len()
            };

            if &_first_list[last_index - 1] == _second_list.last().unwrap() {
                break;
            } else {
                _second_list.remove(start_index);
            }
        }

        let assert_to_list = &_first_list[start_index..last_index];

        if _second_list == assert_to_list {
            Comparison::Superlist
        } else {
            Comparison::Unequal
        }
    } else if _first_list == _second_list {
        if original_first.len() < original_second.len() {
            Comparison::Sublist
        } else if original_first.len() > original_second.len() {
            Comparison::Superlist
        } else {
            Comparison::Equal
        }
    } else {
        Comparison::Unequal
    }
}
