pub fn edit_distance(source: &str, target: &str) -> usize {
    let len_source = source.chars().count();
    let len_target = target.chars().count();

    let mut DP: Vec<Vec<usize>> = vec![vec![0 as usize; len_target + 1]; len_source + 1];

    for i in 1..len_source {
        DP[i][0] = i;
    }

    for j in 1..len_target {
        DP[0][j] = j;
    }

    let mut substitution_cost: usize;

    for j in 1..=len_target {
        for i in 1..=len_source {
            if source.chars().nth(i) == target.chars().nth(j) {
                substitution_cost = 0;
            } else {
                substitution_cost = 1;
            }

            DP[i][j] = std::cmp::min(DP[i - 1][j] + 1,
                std::cmp::min(DP[i][j - 1] + 1, DP[i - 1][j - 1] + substitution_cost)
            );
        }
    }

    return DP[len_source][len_target];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = edit_distance("kitten", "sitting");
        assert_eq!(result, 3);
    }
}
