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
    pub receipt: Vec<f64>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart { items: Vec::new(), receipt: Vec::new() }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        for item in s.products{
            if item.0 == ele{
                self.items.push(item);
            }
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        self.items.sort_by(|a, b| -> std::cmp::Ordering {a.1.cmp(b.1)});
        for item in self.items{
            
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
