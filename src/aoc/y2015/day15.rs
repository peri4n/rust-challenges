use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{alpha1, i32},
    multi::separated_list1,
    sequence::{preceded, terminated},
};

use crate::aoc::Solution;

pub const INPUT: &str = include_str!("day15.txt");

pub struct Day15 {
    ingredients: Vec<Ingredient>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    pub fn score_capacity(&self, amount: i32) -> i32 {
        self.capacity * amount
    }

    pub fn score_durability(&self, amount: i32) -> i32 {
        self.durability * amount
    }

    pub fn score_flavor(&self, amount: i32) -> i32 {
        self.flavor * amount
    }

    pub fn score_texture(&self, amount: i32) -> i32 {
        self.texture * amount
    }
}

impl Solution for Day15 {
    type Fst = i32;
    type Snd = i32;

    fn parse(input: &str) -> Self {
        let ingredients = parse_input(input)
            .expect("Should have been able to parse the input")
            .1;
        Day15 { ingredients }
    }

    fn fst(&self) -> i32 {
        let ingredients = &self.ingredients;

        let mut max = 0;

        for sugar in 0..=100 {
            for sprinkles in 0..=(100 - sugar) {
                for candy in 0..=(100 - sugar - sprinkles) {
                    let chocolate = 100 - sugar - sprinkles - candy;
                    let capacity_score = ingredients[0].score_capacity(sugar)
                        + ingredients[1].score_capacity(sprinkles)
                        + ingredients[2].score_capacity(candy)
                        + ingredients[3].score_capacity(chocolate);

                    let durability_score = ingredients[0].score_durability(sugar)
                        + ingredients[1].score_durability(sprinkles)
                        + ingredients[2].score_durability(candy)
                        + ingredients[3].score_durability(chocolate);

                    let flavor_score = ingredients[0].score_flavor(sugar)
                        + ingredients[1].score_flavor(sprinkles)
                        + ingredients[2].score_flavor(candy)
                        + ingredients[3].score_flavor(chocolate);

                    let texture_score = ingredients[0].score_texture(sugar)
                        + ingredients[1].score_texture(sprinkles)
                        + ingredients[2].score_texture(candy)
                        + ingredients[3].score_texture(chocolate);

                    let score = capacity_score.max(0)
                        * durability_score.max(0)
                        * flavor_score.max(0)
                        * texture_score.max(0);

                    max = score.max(max);
                }
            }
        }

        max
    }

    fn snd(&self) -> i32 {
        let ingredients = &self.ingredients;

        let mut max = 0;

        for sugar in 0..=100 {
            for sprinkles in 0..=(100 - sugar) {
                for candy in 0..=(100 - sugar - sprinkles) {
                    let chocolate = 100 - sugar - sprinkles - candy;

                    let calories = ingredients[0].calories * sugar
                        + ingredients[1].calories * sprinkles
                        + ingredients[2].calories * candy
                        + ingredients[3].calories * chocolate;

                    if calories != 500 {
                        continue;
                    }

                    let capacity_score = ingredients[0].score_capacity(sugar)
                        + ingredients[1].score_capacity(sprinkles)
                        + ingredients[2].score_capacity(candy)
                        + ingredients[3].score_capacity(chocolate);

                    let durability_score = ingredients[0].score_durability(sugar)
                        + ingredients[1].score_durability(sprinkles)
                        + ingredients[2].score_durability(candy)
                        + ingredients[3].score_durability(chocolate);

                    let flavor_score = ingredients[0].score_flavor(sugar)
                        + ingredients[1].score_flavor(sprinkles)
                        + ingredients[2].score_flavor(candy)
                        + ingredients[3].score_flavor(chocolate);

                    let texture_score = ingredients[0].score_texture(sugar)
                        + ingredients[1].score_texture(sprinkles)
                        + ingredients[2].score_texture(candy)
                        + ingredients[3].score_texture(chocolate);

                    let score = capacity_score.max(0)
                        * durability_score.max(0)
                        * flavor_score.max(0)
                        * texture_score.max(0);

                    max = score.max(max);
                }
            }
        }

        max
    }
}

fn parse_input(content: &str) -> IResult<&str, Vec<Ingredient>> {
    separated_list1(tag("\n"), parse_ingredient)(content)
}

fn parse_ingredient(content: &str) -> IResult<&str, Ingredient> {
    let (content, name) = terminated(alpha1, tag(": "))(content)?;
    let (content, capacity) = terminated(preceded(tag("capacity "), i32), tag(", "))(content)?;
    let (content, durability) = terminated(preceded(tag("durability "), i32), tag(", "))(content)?;
    let (content, flavor) = terminated(preceded(tag("flavor "), i32), tag(", "))(content)?;
    let (content, texture) = terminated(preceded(tag("texture "), i32), tag(", "))(content)?;
    let (content, calories) = preceded(tag("calories "), i32)(content)?;

    Ok((
        content,
        Ingredient {
            name: name.to_string(),
            capacity,
            durability,
            flavor,
            texture,
            calories,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        let day = Day15::parse(INPUT);
        assert_eq!(day.fst(), 222870);
        assert_eq!(day.snd(), 117936);
    }
}
