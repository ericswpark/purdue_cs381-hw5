use std::cmp::min;

pub fn valid_tours(b: &[u32]) -> u32 {
    if b.len() == 0 {
        return 0;
    }

    let mut t = Vec::new();

    for (day, _rest_days_needed) in b.iter().enumerate() {
        t.push(1);

        if day == 0 {
            // Base case
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

#[derive(Clone)]
struct DuneMerge {
    cost: u32,
    new_dune_size: u32,
}

pub fn sand_dunes_merging(cost: &[u32]) -> u32 {
    if cost.len() == 0 {
        return 0;
    }

    let mut t = vec![
        vec![
            DuneMerge {
                cost: u32::MAX,
                new_dune_size: 0
            };
            cost.len()
        ];
        cost.len()
    ];

    // merging `merge_count` other elements to current dune
    for merge_count in 0..cost.len() {
        for (index, dune_size) in cost.iter().enumerate() {
            if index + merge_count >= cost.len() {
                break;
            }

            // Base case - dune alone
            if merge_count == 0 {
                t[index][index].cost = *dune_size;
                t[index][index].new_dune_size = *dune_size;
                continue;
            }

            for sub_merge in 0..merge_count {
                let mut cost: u32 = 0;
                let left_side_start_index = index;
                let left_side_end_index = index + sub_merge;
                let left_side_cost = t[left_side_start_index][left_side_end_index].cost;
                let left_side_new_dune_size = t[left_side_start_index][left_side_end_index].new_dune_size;
                if left_side_start_index != left_side_end_index {
                    cost = cost.saturating_add(left_side_new_dune_size);
                }

                let right_side_start_index = index + sub_merge + 1;
                let right_side_end_index = index + merge_count;
                let right_side_cost = t[right_side_start_index][right_side_end_index].cost;
                let right_side_new_dune_size =
                    t[right_side_start_index][right_side_end_index].new_dune_size;
                if right_side_start_index != right_side_end_index {
                    cost = cost.saturating_add(right_side_new_dune_size);
                }

                t[left_side_start_index][right_side_end_index].cost = min(
                    t[index][index + merge_count].cost,
                    cost.saturating_add(left_side_cost).saturating_add(right_side_cost),
                );
                t[left_side_start_index][right_side_end_index].new_dune_size =
                    left_side_new_dune_size.saturating_add(right_side_new_dune_size);
            }
        }
    }

    t[0][cost.len() - 1].cost
}


pub fn greedy_sand_dune_merging(cost: &[u32]) -> u32 {
    let mut merged = cost.to_vec();
    let mut cost = 0;

    while merged.len() > 1 {
        let mut consecutive_sum = u32::MAX;
        let mut min_index = 0;

        // Find smallest two items
        for (index, dune) in merged.iter().enumerate() {
            if index + 1 < merged.len() {
                if dune + merged[index + 1] < consecutive_sum {
                    min_index = index;
                    consecutive_sum = dune + merged[index + 1];
                }
            }
        }

        // Merge
        merged[min_index] = consecutive_sum;
        merged.remove(min_index + 1);
        cost += consecutive_sum;
    }

    cost
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

    #[test]
    fn test_sand_dunes_merging() {
        assert_eq!(sand_dunes_merging(&[1, 1]), 2);
        assert_eq!(sand_dunes_merging(&[1, 1, 1]), 5);
        assert_eq!(sand_dunes_merging(&[3, 5, 1]), 15);
        assert_eq!(sand_dunes_merging(&[1, 1, 1, 1]), 8);
        assert_eq!(sand_dunes_merging(&[10, 1, 1, 10]), 36);
    }

    #[test]
    fn test_sand_dunes_merging_meet_patel() {
        assert_eq!(sand_dunes_merging(&[4, 8, 6, 3, 1, 9]), 76);
    }

    #[test]
    fn test_greedy_sand_dunes_merging() {
        assert_eq!(greedy_sand_dune_merging(&[1, 1]), 2);
        assert_eq!(greedy_sand_dune_merging(&[1, 1, 1]), 5);
        assert_eq!(greedy_sand_dune_merging(&[3, 5, 1]), 15);
        assert_eq!(greedy_sand_dune_merging(&[1, 1, 1, 1]), 8);
        assert_eq!(greedy_sand_dune_merging(&[10, 1, 1, 10]), 36);
    }

    #[test]
    fn test_different_merge_strategy() {
        assert_eq!(sand_dunes_merging(&[4, 8, 2, 8]), 44);
        assert_eq!(greedy_sand_dune_merging(&[4, 8, 2, 8]), 46);
    }
}
