use json;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut cals = 0.0;
    let mut carbs = 0.0;
    let mut proteins = 0.0;
    let mut fats = 0.0;

    for food in foods {
        let cals_text = food.calories[1].clone();
        let cals_value = &cals_text[0 .. cals_text.len() - 4];
        cals += cals_value.parse::<f64>().unwrap() * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
    };

    let result = json::object!{
        cals: (cals * 100.0).round() / 100.0,
        carbs: (carbs * 100.0).round() / 100.0,
        proteins: (proteins * 100.0).round() / 100.0,
        fats: (fats * 100.0).round() / 100.0,
    };

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = vec![
        Food {
            name: String::from("big mac"),
            calories: ["2133.84kJ".to_string(), "510kcal".to_string()],
            proteins: 27.0,
            fats: 26.0,
            carbs: 41.0,
            nbr_of_portions: 2.0,
        },
        Food {
            name: "pizza margherita".to_string(),
            calories: ["1500.59kJ".to_string(), "358.65kcal".to_string()],
            proteins: 13.89,
            fats: 11.21,
            carbs: 49.07,
            nbr_of_portions: 4.9,
        },
    ];

        let right = json::object! {
            cals: 2777.39,
            carbs: 322.44,
            proteins: 122.06,
            fats: 106.93, 
        };

        assert_eq!(calculate_macros(a), right);
    }
}
