use std::collections::HashMap;

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut map = HashMap::new();
    for word in words{
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    return map;
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    return frequency_count.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sentence = "this is a very basic sentence with only few \
                repetitions. once again this is very basic and \
                but it should be enough for basic tests".to_string();
    let words = sentence.split(" ").collect::<Vec<&str>>();
    let frequency_count = word_frequency_counter(words);
        assert_eq!(nb_distinct_words(&frequency_count), 20 as usize);
    }
}
