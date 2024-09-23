use std::cmp::min;

pub fn valid_tours(b: &[u32]) -> u32 {
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
    let mut t = vec![
        vec![
            DuneMerge {
                cost: u32::MAX / 3,
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
                let left_side_cost = t[index][index + sub_merge].cost;
                let left_side_new_dune_size = t[index][index + sub_merge].new_dune_size;
                if index + sub_merge != index {
                    cost += left_side_new_dune_size;
                }

                let right_side_cost = t[index + sub_merge + 1][index + merge_count].cost;
                let right_side_new_dune_size =
                    t[index + sub_merge + 1][index + merge_count].new_dune_size;
                if index + sub_merge + 1 != index + merge_count {
                    cost += right_side_new_dune_size;
                }

                t[index][index + merge_count].cost = min(
                    t[index][index + merge_count].cost,
                    cost + left_side_cost + right_side_cost,
                );
                t[index][index + merge_count].new_dune_size =
                    left_side_new_dune_size + right_side_new_dune_size;
            }
        }
    }

    // for row in t.iter() {
    //     for col in row.iter() {
    //         if (*col).0 == u32::MAX / 3 {
    //             print!("- ");
    //             continue;
    //         }
    //         print!("{} ", col.0);
    //     }
    //     println!();
    // }

    t[0][cost.len() - 1].cost
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
}
