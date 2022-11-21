pub struct One {
    pub first_layer: Option<Two>
}
#[derive(Clone, Copy)]
pub struct Two {
    pub second_layer: Option<Three>
}
#[derive(Clone, Copy)]
pub struct Three {
    pub third_layer: Option<Four>
}
#[derive(Clone, Copy)]
pub struct Four {
    pub fourth_layer: Option<u16>
}

impl One {
    pub fn get_fourth_layer(&self) -> Option<u16> {
        return self.first_layer?.second_layer?.third_layer?.fourth_layer;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = One {
            first_layer : Some(Two {
                second_layer: Some(Three {
                    third_layer: Some(Four {
                        fourth_layer: Some(1000)
                    })
                })
            })
        };
        assert_eq!(a.get_fourth_layer(), Some(1000));
    }
}
