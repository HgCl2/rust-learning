pub fn mean(list: &Vec<i32>) -> f64 {
    let mut res: f64 = 0.0;
    for num in list {
        res += *num as f64;
    }

    res = res / list.len() as f64;

    return res;
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut list = list.clone();
    list.sort();
    let length = list.len();

    if length % 2 == 0 {
        return list[length / 2] + list[length / 2 + 1];
    } else {
        return list[length / 2];
    }
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut counter: Vec<i32> = vec![0; 100];
    for num in list {
        counter[*num as usize] += 1;
    }

    let mut max: i32 = 0;
    let mut max_ind: usize = 0;
    for (ind, num) in counter.iter().enumerate() {
        if *num > max {
            max = *num;
            max_ind = ind;
        }
    }

    return max_ind as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = vec![4, 7, 5, 2, 5, 1, 3];
        assert_eq!(mean(&v), 3.857142857142857);
        assert_eq!(median(&v), 4);
        assert_eq!(mode(&v), 5);
    }
}
