pub fn is_valid(code: &str) -> bool {
    let mut numbers: Vec<u8> = Vec::new();

    let digits_string: String = code.chars().filter(|c| !c.is_whitespace()).collect();
    let digits_str_vec: Vec<&str> = digits_string.split("").collect();

    let formatted_digits = digits_str_vec[1..digits_str_vec.len() - 1].to_vec();

    for digit in formatted_digits {
        numbers.push(match digit.parse::<u8>() {
            Ok(v) => v,
            Err(_) => {
                return false;
            }
        });
    }

    println!("{:?}", numbers);

    let mut numbers_to_sum: Vec<u8> = numbers.clone();

    for (i, n) in numbers.iter().enumerate().rev() {
        let index = numbers.len() - i;

        if index % 2 == 0 {
            numbers_to_sum[i] = get_calculated_number(n);
        }
    }

    let sum: u8 = numbers_to_sum.iter().sum();

    sum % 10 == 0 && numbers_to_sum.len() > 1
}

fn get_calculated_number(number: &u8) -> u8 {
    let doubled = *number * 2;

    if doubled > 9 {
        doubled - 9
    } else {
        doubled
    }
}
