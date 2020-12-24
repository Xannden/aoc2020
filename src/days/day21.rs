use std::collections::HashMap;

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}
pub fn run() {
    let data = std::fs::read_to_string("input/day21.txt").unwrap();

    let data = parse(&data);

    // println!("{:#?}", data);

    part1(&data);
    part2(&data);
}

fn parse_food(line: &str) -> Food {
    let mut sections = line.split('(');

    let ingredients = sections
        .next()
        .unwrap()
        .split(' ')
        .filter(|i| !i.is_empty())
        .map(|s| s.to_string())
        .collect();

    let allergens = sections.next().unwrap()[9..]
        .split(", ")
        .map(|a| a.trim_end_matches(')'))
        .map(|s| s.to_string())
        .collect();

    Food {
        ingredients,
        allergens,
    }
}

fn parse(input: &str) -> Vec<Food> {
    input.lines().map(|line| parse_food(line)).collect()
}

fn part1(foods: &[Food]) {
    println!("Part 1");

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for food in foods {
        for allergen in &food.allergens {
            map.entry(allergen.clone())
                .and_modify(|ing| {
                    let mut i = 0;
                    while i != ing.len() {
                        if !food.ingredients.contains(&ing[i]) {
                            ing.remove(i);
                        } else {
                            i += 1;
                        }
                    }
                })
                .or_insert_with(|| food.ingredients.clone());
        }
    }

    let possible_allergens = map.values().flatten().collect::<Vec<_>>();

    let mut nonallergens = foods
        .iter()
        .flat_map(|food| &food.ingredients)
        .filter(|ingredient| !possible_allergens.contains(ingredient))
        .collect::<Vec<_>>();
    nonallergens.sort_unstable();
    nonallergens.dedup();

    let mut count = 0;

    for food in foods {
        for ingredient in &nonallergens {
            if food.ingredients.contains(ingredient) {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

fn part2(foods: &[Food]) {
    println!("Part 2");

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for food in foods {
        for allergen in &food.allergens {
            map.entry(allergen.clone())
                .and_modify(|ing| {
                    let mut i = 0;
                    while i != ing.len() {
                        if !food.ingredients.contains(&ing[i]) {
                            ing.remove(i);
                        } else {
                            i += 1;
                        }
                    }
                })
                .or_insert_with(|| food.ingredients.clone());
        }
    }

    // println!("{:#?}", map);

    let mut mapping = HashMap::new();

    loop {
        let result = map
            .iter()
            .find(|v| v.1.len() == 1)
            .map(|v| (v.0.clone(), v.1.clone()));

        if let Some((ing, v)) = result {
            mapping.insert(ing, v[0].clone());

            for (_, allergens) in map.iter_mut() {
                if let Some(idx) = allergens.iter().position(|a| *a == v[0]) {
                    allergens.remove(idx);
                }
            }
        } else {
            break;
        }
    }

    let mut result = mapping.iter().collect::<Vec<_>>();

    result.sort_by(|lhs, rhs| lhs.0.cmp(rhs.0));

    let result = result.iter().map(|v| v.1).cloned().collect::<Vec<_>>().join(",");

    println!("{:#?}", result);
}
