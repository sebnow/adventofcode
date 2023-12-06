use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

fn translate(food: &[Food]) -> HashMap<String, String> {
    let mut allergens: HashMap<String, HashSet<String>> = HashMap::new();

    // Find possible mapping
    for f in food {
        let mut ingredients: HashSet<String> = HashSet::new();
        f.ingredients.iter().for_each(|i| {
            ingredients.insert(i.to_string());
        });

        for a in &f.allergens {
            let prev = allergens.entry(a.to_owned()).or_insert(ingredients.clone());
            *prev = prev
                .intersection(&ingredients)
                .map(|i| i.to_owned())
                .collect();
        }
    }

    let mut translated = HashMap::new();
    let mut queue: VecDeque<String> = allergens.keys().map(|k| k.to_owned()).collect();
    while let Some(allergen) = queue.pop_front() {
        let old = allergens.clone();
        let is = old.get(&allergen).unwrap();
        if is.len() != 1 {
            queue.push_back(allergen);
            continue;
        }

        let ingredient = is.iter().next().unwrap();
        translated.insert(allergen.clone(), ingredient.clone());

        for (_, ingredients) in &mut allergens {
            ingredients.remove(ingredient);
        }
    }

    translated
}

fn parse_input<'a>(s: &'a str) -> impl Iterator<Item = Food> + 'a {
    s.lines().map(|l| {
        let mut parts = l.split(" (contains ");
        let ingredients = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.to_string())
            .collect();
        let allergens = parts
            .next()
            .map(|p| {
                p[0..p.len() - 1]
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        Food {
            ingredients,
            allergens,
        }
    })
}

fn part_one(input: &str) -> String {
    let food: Vec<Food> = parse_input(input).collect();
    let translated = translate(&food);
    let allergens: HashSet<String> = translated.values().map(|v| v.to_owned()).collect();

    food.iter()
        .flat_map(|f| f.ingredients.iter().filter(|&i| !allergens.contains(i)))
        .count()
        .to_string()
}

fn part_two(input: &str) -> String {
    let food: Vec<Food> = parse_input(input).collect();
    let mut ingredients: Vec<(String, String)> = translate(&food).iter().map(|(k, v)| (k.to_owned(),v.to_owned())).collect();
    ingredients.sort_by_key(|(a, _)| a.to_owned());
    let canon: Vec<String> = ingredients.iter().map(|(_, i)| i.to_owned()).collect();
    canon.join(",")
}

fn main() {
    let input = include_str!("../../../../input/2020/day21.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 21, 1, 1);
    test_example!(example_two_1, part_two, 21, 2, 1);
}
