fn main() {
    println!("hej");
}

#[allow(dead_code)]
fn calculate_sum_from_string(input: String) -> i32 {
    if !input.is_empty() {
        let mut sum: i32 = 0;
        let number_strings = input.split(',');
        for number_string in number_strings {
            let number: i32 = number_string.trim().parse().expect("Should be a number");
            sum += number;
        }
        return sum;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_empty_string_return_zero() {
        assert_eq!(calculate_sum_from_string(String::from("")), 0)
    }

    #[test]
    fn when_one_digit_should_return_that_number() {
        assert_eq!(calculate_sum_from_string(String::from("1")), 1);
    }

    #[test]
    fn when_spaces_around_digit_should_return_that_number() {
        assert_eq!(calculate_sum_from_string(String::from(" 1 ")), 1);
    }

    #[test]
    fn when_two_digits_comma_separated_should_return_sum() {
        assert_eq!(calculate_sum_from_string(String::from("1,2")), 3);
    }

    #[test]
    fn when_spaces_around_two_digits_should_return_sum() {
        assert_eq!(calculate_sum_from_string(String::from(" 1,2 ")), 3);
    }

    #[test]
    fn when_two_multi_digit_numbers_should_return_sum() {
        assert_eq!(calculate_sum_from_string(String::from("11,22")), 33);
    }
}
