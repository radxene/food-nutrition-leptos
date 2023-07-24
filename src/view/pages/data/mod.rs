use serde::{de, Deserialize, Deserializer};
use serde_json::{json, Value};

#[derive(Debug, Default, Deserialize, Clone)]
pub struct NutritionDetail {
    #[serde(deserialize_with = "de_string_as_f64")]
    pub amount: f64,
    pub unit: String,
    pub alias: String,
    pub derivation: String,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct FoodNutrition {
    pub water: Option<NutritionDetail>,
    pub energy: Option<NutritionDetail>,
    pub protein: Option<NutritionDetail>,
    pub lipid: Option<NutritionDetail>,
    pub cholesterol: Option<NutritionDetail>,
    pub sodium: Option<NutritionDetail>,
    pub potassium: Option<NutritionDetail>,
    pub carbohydrate: Option<NutritionDetail>,
    pub fiber: Option<NutritionDetail>,
    pub sugars: Option<NutritionDetail>,
    pub iron: Option<NutritionDetail>,
    pub magnesium: Option<NutritionDetail>,
    pub calcium: Option<NutritionDetail>,
    pub vitamin_c: Option<NutritionDetail>,
    pub vitamin_d: Option<NutritionDetail>,
    pub vitamin_b6: Option<NutritionDetail>,
    pub vitamin_b12: Option<NutritionDetail>,
}

fn de_string_as_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        Value::Number(num) => num
            .as_f64()
            .ok_or_else(|| de::Error::custom("Invalid number"))?,
        _ => return Err(de::Error::custom("wrong type")),
    })
}

pub fn get_food_data() -> Value {
    json!([
        {
          "kind": "chicken",
          "portion": { "amount": "100", "unit": "g" },
          "nutrition": {
            "water": { "amount": "59.4", "unit": "g", "alias": "", "derivation": "" },
            "energy": { "amount": "239", "unit": "kcal", "alias": "", "derivation": "" },
            "protein": { "amount": "27.3", "unit": "g", "alias": "", "derivation": "" },
            "lipid": { "amount": "13.6", "unit": "g", "alias": "", "derivation": "" },
            "cholesterol": { "amount": "88", "unit": "mg", "alias": "", "derivation": "" },
            "sodium": { "amount": "82", "unit": "mg", "alias": "Na", "derivation": "" },
            "potassium": { "amount": "223", "unit": "mg", "alias": "K", "derivation": "" },
            "carbohydrate": { "amount": "0", "unit": "g", "alias": "", "derivation": "Assumed zero" },
            "fiber": { "amount": "0", "unit": "g", "alias": "", "derivation": "Assumed zero" },
            "sugars": { "amount": "0", "unit": "g", "alias": "", "derivation": "Assumed zero" },
            "iron": { "amount": "1.26", "unit": "mg", "alias": "Fe", "derivation": "" },
            "magnesium": { "amount": "23", "unit": "mg", "alias": "Mg", "derivation": "" },
            "calcium": { "amount": "15", "unit": "mg", "alias": "Ca", "derivation": "" },
            "vitamin_c": { "amount": "0", "unit": "mg", "alias": "", "derivation": "" },
            "vitamin_d": { "amount": "2", "unit": "IU", "alias": "", "derivation": "" },
            "vitamin_b6": { "amount": "0.4", "unit": "mg", "alias": "", "derivation": "" },
            "vitamin_b12": { "amount": "0.3", "unit": "µg", "alias": "", "derivation": "" }
          },
          "source": "https://fdc.nal.usda.gov/fdc-app.html#/food-details/171450/nutrients"
        },
        {
          "kind": "beef",
          "portion": { "amount": "100", "unit": "g" },
          "nutrition": {
            "water": { "amount": "58", "unit": "g", "alias": "", "derivation": "" },
            "energy": { "amount": "250", "unit": "kcal", "alias": "", "derivation": "" },
            "protein": { "amount": "25.9", "unit": "g", "alias": "", "derivation": "" },
            "lipid": { "amount": "15.4", "unit": "g", "alias": "", "derivation": "" },
            "cholesterol": { "amount": "80", "unit": "mg", "alias": "", "derivation": "" },
            "sodium": { "amount": "72", "unit": "mg", "alias": "Na", "derivation": "" },
            "potassium": { "amount": "318", "unit": "mg", "alias": "K", "derivation": "" },
            "carbohydrate": { "amount": "0", "unit": "g", "alias": "", "derivation": "Assumed zero" },
            "fiber": { "amount": "0", "unit": "g", "alias": "", "derivation": "Assumed zero" },
            "sugars": { "amount": "0", "unit": "g", "alias": "", "derivation": "Assumed zero" },
            "iron": { "amount": "2.6", "unit": "mg", "alias": "Fe", "derivation": "" },
            "magnesium": { "amount": "21", "unit": "mg", "alias": "Mg", "derivation": "" },
            "calcium": { "amount": "18", "unit": "mg", "alias": "Ca", "derivation": "" },
            "vitamin_c": { "amount": "0", "unit": "mg", "alias": "", "derivation": "" },
            "vitamin_d": { "amount": "2", "unit": "IU", "alias": "", "derivation": "" },
            "vitamin_b6": { "amount": "0.382", "unit": "mg", "alias": "", "derivation": "" },
            "vitamin_b12": { "amount": "2.64", "unit": "µg", "alias": "", "derivation": "" }
          },
          "source": "https://fdc.nal.usda.gov/fdc-app.html#/food-details/174032/nutrients"
        },
        {
          "kind": "pork",
          "portion": { "amount": "100", "unit": "g" },
          "nutrition": {
            "water": { "amount": "57.9", "unit": "g", "alias": "", "derivation": "" },
            "energy": { "amount": "242", "unit": "kcal", "alias": "", "derivation": "" },
            "protein": { "amount": "27.3", "unit": "g", "alias": "", "derivation": "" },
            "lipid": { "amount": "14", "unit": "g", "alias": "", "derivation": "" },
            "cholesterol": { "amount": "80", "unit": "mg", "alias": "", "derivation": "" },
            "sodium": { "amount": "62", "unit": "mg", "alias": "Na", "derivation": "" },
            "potassium": { "amount": "423", "unit": "mg", "alias": "K", "derivation": "" },
            "carbohydrate": { "amount": "0", "unit": "g", "alias": "", "derivation": "Assumed zero" },
            "fiber": { "amount": "0", "unit": "g", "alias": "", "derivation": "Assumed zero" },
            "sugars": { "amount": "0", "unit": "g", "alias": "", "derivation": "Assumed zero" },
            "iron": { "amount": "0.9", "unit": "mg", "alias": "Fe", "derivation": "" },
            "magnesium": { "amount": "28", "unit": "mg", "alias": "Mg", "derivation": "" },
            "calcium": { "amount": "19", "unit": "mg", "alias": "Ca", "derivation": "" },
            "vitamin_c": { "amount": "0.6", "unit": "mg", "alias": "", "derivation": "" },
            "vitamin_d": { "amount": "53", "unit": "IU", "alias": "", "derivation": "" },
            "vitamin_b6": { "amount": "0.5", "unit": "mg", "alias": "", "derivation": "" },
            "vitamin_b12": { "amount": "0.7", "unit": "µg", "alias": "", "derivation": "" }
          },
          "source": "https://fdc.nal.usda.gov/fdc-app.html#/food-details/167820/nutrients"
        },
        {
          "kind": "apple",
          "portion": { "amount": "100", "unit": "g" },
          "nutrition": {
            "water": { "amount": "88.1", "unit": "g", "alias": "", "derivation": "" },
            "energy": { "amount": "48", "unit": "kcal", "alias": "", "derivation": "" },
            "protein": { "amount": "0.09", "unit": "g", "alias": "", "derivation": "" },
            "lipid": { "amount": "0.29", "unit": "g", "alias": "", "derivation": "" },
            "cholesterol": null,
            "sodium": { "amount": "5", "unit": "mg", "alias": "Na", "derivation": "" },
            "potassium": { "amount": "96", "unit": "mg", "alias": "K", "derivation": "" },
            "carbohydrate": { "amount": "11.4", "unit": "g", "alias": "", "derivation": "Calculated" },
            "fiber": null,
            "sugars": { "amount": "10.3", "unit": "g", "alias": "", "derivation": "Summed" },
            "iron": { "amount": "0.04", "unit": "mg", "alias": "Fe", "derivation": "" },
            "magnesium": { "amount": "4.9", "unit": "mg", "alias": "Mg", "derivation": "" },
            "calcium": { "amount": "7", "unit": "mg", "alias": "Ca", "derivation": "" },
            "vitamin_c": { "amount": "51.2", "unit": "mg", "alias": "", "derivation": "" },
            "vitamin_d": null,
            "vitamin_b6": { "amount": "0.014", "unit": "mg", "alias": "", "derivation": "" },
            "vitamin_b12": null
          },
          "source": "https://fdc.nal.usda.gov/fdc-app.html#/food-details/2003590/nutrients"
        },
        {
          "kind": "banana",
          "portion": { "amount": "100", "unit": "g" },
          "nutrition": {
            "water": { "amount": "78.3", "unit": "g", "alias": "", "derivation": "" },
            "energy": { "amount": "85", "unit": "kcal", "alias": "", "derivation": "" },
            "protein": { "amount": "0.73", "unit": "g", "alias": "", "derivation": "" },
            "lipid": { "amount": "0.22", "unit": "g", "alias": "", "derivation": "" },
            "cholesterol": null,
            "sodium": null,
            "potassium": null,
            "carbohydrate": { "amount": "20.1", "unit": "g", "alias": "", "derivation": "Calculated" },
            "fiber": null,
            "sugars": { "amount": "15.8", "unit": "g", "alias": "", "derivation": "Summed" },
            "iron": null,
            "magnesium": null,
            "calcium": null,
            "vitamin_c": { "amount": "9.7", "unit": "mg", "alias": "", "derivation": "" },
            "vitamin_d": null,
            "vitamin_b6": { "amount": "0.234", "unit": "mg", "alias": "", "derivation": "" },
            "vitamin_b12": null
          },
          "source": "https://fdc.nal.usda.gov/fdc-app.html#/food-details/1105073/nutrients"
        }
    ])
}
