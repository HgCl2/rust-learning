pub use case;
pub use case::CaseExt;

pub fn edit_distance(source: &str, target: &str) -> usize {
    let len_source = source.chars().count();
    let len_target = target.chars().count();

    let mut dp: Vec<Vec<usize>> = vec![vec![0 as usize; len_target + 1]; len_source + 1];

    for i in 1..(len_source+1){
        dp[i][0] = i;
    }

    for j in 1..(len_target+1) {
        dp[0][j] = j;
    }

    let mut substitution_cost: usize;
    for j in 1..=len_target {
        for i in 1..=len_source {
            if source.chars().nth(i-1).unwrap() == target.chars().nth(j-1).unwrap() {
                substitution_cost = 0;
            } else {
                substitution_cost = 1;
            }

            dp[i][j] = std::cmp::min(
                dp[i - 1][j] + 1,
                std::cmp::min(dp[i][j - 1] + 1,
                dp[i - 1][j - 1] + substitution_cost)
            );
        }
    }

    return dp[len_source][len_target];
}

pub fn expected_variable(target_str: &str, expected_str: &str) -> Option<String> {
    let target: String = target_str.to_lowercase();
    let expected: String = expected_str.to_lowercase();

    if !target.is_camel_lowercase() && target != target.to_snake() && 
        target.contains(|c: char| c != '_' && c.is_ascii_punctuation()){
        return  None;
    }else if target_str == expected_str{
        return Some("100%".to_string());
    }

    let differ_chars = edit_distance(&target, &expected);    
    let alikeness = 1.0 - differ_chars as f64 / expected.len() as f64;
    let alikeness = (alikeness * 100.0).round();
    
    if alikeness > 50.0{
        return Some(format!("{alikeness}%"));
    }

    return None;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(expected_variable("On_Point", "on_point").unwrap(), "100%".to_string());
        assert_eq!(expected_variable("soClose", "So_Close").unwrap(), "88%".to_string());
        assert_eq!(expected_variable("something", "something_completely_different"), None);
        assert_eq!(expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch").unwrap(), "67%".to_string());
        assert_eq!(expected_variable("", ""), None);
    }
}
