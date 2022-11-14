
pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

pub fn at_index(vec: &Vec<String>, index: usize) -> String {
    return vec[index].clone();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let mut groceries = vec![
		    "yogurt".to_string(),
		    "panettone".to_string(),
		    "bread".to_string(),
		    "cheese".to_string(),
	    ];
	    insert(&mut groceries, String::from("nuts"));
	    println!("The groceries list contains {:?}", &groceries);
        assert_eq!(groceries[4], String::from("nuts"));
    }

    #[test]
    fn test1() {
        let groceries = vec![
		    "yogurt".to_string(),
		    "panettone".to_string(),
		    "bread".to_string(),
		    "cheese".to_string(),
	    ];
        assert_eq!(at_index(&groceries, 1), groceries[1]);
    }
}
