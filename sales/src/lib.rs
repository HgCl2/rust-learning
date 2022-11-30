use std::cmp::Ordering;
#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart { items: Vec::new(), receipt: Vec::new() }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        for item in &s.products{
            if item.0 == ele{
                self.items.push(item.clone());
            }
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut result: Vec<f32> = Vec::new();
        let mut items_arr = self.items.clone();

        items_arr.sort_by(|a, b| -> std::cmp::Ordering {cmp_f32(a.1, b.1)});
        let sorted_items = items_arr.clone();
        
        let free_items = items_arr.len() / 3;
        for ind in 0..free_items{
            items_arr[ind].1 = 0.0;
        }

        let new_coeff = calculate_coeff(items_arr, self.items.clone());

        for item in sorted_items{
            let new_price = item.1 * new_coeff;
            result.push(round2d(new_price));
        }

        self.receipt = result.clone();
        return result;
    }

}

pub fn round2d(num: f32) -> f32{
    return (num * 100.0).round() / 100.0;
}

pub fn calculate_coeff(new_items: Vec<(String, f32)>, old_items: Vec<(String, f32)>) -> f32 {
    let mut new_summ = 0.0;
    let mut old_summ = 0.0;

    for item in new_items{
        new_summ += item.1;
    }

    for item in old_items{
        old_summ += item.1;
    }

    return 1.0 - (old_summ - new_summ) / old_summ;
}



pub fn cmp_f32(first: f32, second: f32) -> Ordering{
    if first > second {
        return Ordering::Greater;
    }else if first < second {
        return Ordering::Less;
    }

    return Ordering::Equal;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
