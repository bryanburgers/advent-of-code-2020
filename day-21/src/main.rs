use std::collections::{HashMap, HashSet};
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let foods: Vec<Food> = input.lines().map(|line| Food::parse(line.trim())).collect();

    let mut solution = Solution::default();
    for food in &foods {
        solution.add_food(food);
    }

    let ingredients_without_allergens = solution.ingredients_without_allergens();
    let mut total_appearances = 0;
    for food in &foods {
        for ingredient in &food.ingredients {
            if ingredients_without_allergens.contains(&ingredient) {
                total_appearances += 1;
            }
        }
    }
    println!("{}", total_appearances);

    let ingredients_with_allergens = solution.ingredients_with_allergens();
    let mut ingredients_with_allergens: Vec<(Allergen, Ingredient)> =
        ingredients_with_allergens.into_iter().collect();
    ingredients_with_allergens.sort();
    let r = ingredients_with_allergens
        .into_iter()
        .map(|(allergen, ingredient)| ingredient.0.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("{}", r);
}

#[derive(Debug, Clone)]
pub struct Food<'a> {
    ingredients: HashSet<Ingredient<'a>>,
    allergens: HashSet<Allergen<'a>>,
}

impl<'a> Food<'a> {
    fn parse(s: &'a str) -> Self {
        let open_paren = s.find('(').unwrap();
        let closed_paren = s.find(')').unwrap();
        let ingredients = &s[0..(open_paren - 1)];
        let allergens = &s[(open_paren + 10)..closed_paren];
        let ingredients = ingredients.split_whitespace().map(Into::into).collect();
        let allergens = allergens.split(", ").map(Into::into).collect();

        Self {
            ingredients,
            allergens,
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Allergen<'a>(&'a str);
impl<'a> From<&'a str> for Allergen<'a> {
    fn from(i: &'a str) -> Self {
        Self(i)
    }
}
impl<'a> std::fmt::Debug for Allergen<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[derive(Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Ingredient<'a>(&'a str);
impl<'a> From<&'a str> for Ingredient<'a> {
    fn from(i: &'a str) -> Self {
        Self(i)
    }
}
impl<'a> std::fmt::Debug for Ingredient<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Default)]
pub struct Solution<'a> {
    all_ingredients: HashSet<Ingredient<'a>>,
    allergens: HashMap<Allergen<'a>, HashSet<Ingredient<'a>>>,
}

impl<'a> Solution<'a> {
    fn add_food(&mut self, food: &Food<'a>) {
        self.all_ingredients = self
            .all_ingredients
            .union(&food.ingredients)
            .copied()
            .collect();
        for &allergen in &food.allergens {
            let ingredients = food.ingredients.clone();
            let entry = self
                .allergens
                .entry(allergen)
                .or_insert_with(|| ingredients);
            *entry = entry.intersection(&food.ingredients).copied().collect();
        }

        self.reduce();
    }

    fn reduce(&mut self) {
        let mut did_reduce = true;
        while did_reduce {
            did_reduce = false;
            let mut reduce = HashMap::new();

            for (&allergen, ingredients) in &self.allergens {
                if ingredients.len() == 1 {
                    let ingredient = ingredients.iter().copied().next().unwrap();
                    reduce.insert(allergen, ingredient);
                }
            }

            for (identified_allergen, identified_ingredient) in reduce {
                for (&allergen, ingredients) in &mut self.allergens {
                    if allergen == identified_allergen {
                        continue;
                    }

                    if ingredients.remove(&identified_ingredient) {
                        did_reduce = true;
                    }
                }
            }
        }
    }

    fn ingredients_with_allergens(&self) -> HashMap<Allergen, Ingredient> {
        let mut hashmap = HashMap::new();
        for (&allergen, ingredients) in &self.allergens {
            if ingredients.len() != 1 {
                panic!("{:?} has more than one ingredient!", allergen);
            }
            hashmap.insert(allergen, ingredients.iter().copied().next().unwrap());
        }
        hashmap
    }

    fn ingredients_without_allergens(&self) -> HashSet<Ingredient> {
        let mut ingredients_with_allergens = HashSet::new();
        for (&allergen, ingredients) in &self.allergens {
            if ingredients.len() != 1 {
                panic!("{:?} has more than one ingredient!", allergen);
            }
            ingredients_with_allergens = ingredients_with_allergens
                .union(ingredients)
                .copied()
                .collect();
        }

        self.all_ingredients
            .difference(&ingredients_with_allergens)
            .copied()
            .collect()
    }
}
