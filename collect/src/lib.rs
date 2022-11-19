pub fn bubble_sort(vec: &mut Vec<i32>) {
    let length = vec.len();
    for j in 1..length {
        let mut flag = 0;
        for i in 0..length-j {
            if vec[i] > vec[i+1]{
                let temp = vec[i];
                vec[i] = vec[i+1];
                vec[i+1] = temp;
                flag = 1;
            }
        }
        if flag == 0{
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ref mut v = vec![3, 2, 4, 5, 1, 7];
	    bubble_sort(v);
        assert_eq!(*v, [1, 2, 3, 4, 5, 7]);
    }
}
