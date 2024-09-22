pub fn valid_tours(b: &[u32]) -> u32 {
    let mut t = Vec::new();

    for (day, _rest_days_needed) in b.iter().enumerate() {
        t.push(1);

        if day == 0 {   // Base case
            continue;
        }

        for prev_days in 0..day {
            if b[prev_days] + (prev_days as u32) < day as u32 {
                t[day] += t[prev_days];
            }
        }
    }

    t.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_tours_handout() {
        let result = valid_tours(&[1, 1, 5]);
        assert_eq!(result, 4);
    }


    #[test]
    fn test_valid_tours_extra_cases() {
        assert_eq!(valid_tours(&[0, 0]), 3);
        assert_eq!(valid_tours(&[0, 0, 0, 0]), 15);
        assert_eq!(valid_tours(&[1, 1, 1, 1]), 7);
        assert_eq!(valid_tours(&[1, 1, 1, 1, 1]), 12);
    }
}