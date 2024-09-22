pub fn valid_tours(b: &[u32]) -> u32 {
    let mut t = Vec::new();

    for (day, _rest_days_needed) in b.iter().enumerate() {
        if day == 0 {
            t.push(1);
            continue;
        }

        let mut acc = 1;

        for prev_days in 0..day {
            if b[prev_days] + (prev_days as u32) < day as u32 {
                acc += t[prev_days];
            }
        }
        t.push(acc);
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
}