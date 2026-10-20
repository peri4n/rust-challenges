use std::collections::{HashMap, HashSet};

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, i32};
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated};
use nom::{IResult, Parser};

use crate::aoc::Solution;
use crate::helper::combinatorics::permute;

pub const INPUT: &str = include_str!("day9.txt");

struct Route {
    from: String,
    to: String,
    cost: i32,
}

fn parse_route(line: &str) -> IResult<&str, Route> {
    let locations = terminated(alpha1, tag(" to ")).and(alpha1);
    let to = preceded(tag(" = "), i32);

    locations
        .and(to)
        .map(|((from, to), cost): ((&str, &str), i32)| Route {
            from: from.to_owned(),
            to: to.to_owned(),
            cost,
        })
        .parse(line)
}

fn parse_routes(definition: &str) -> Vec<Route> {
    separated_list1(tag("\n"), parse_route)
        .parse(definition)
        .expect("Could not parse input file")
        .1
}

fn cost(cities: Vec<&str>, costs: &HashMap<(&str, &str), i32>) -> i32 {
    if cities.len() <= 1 {
        return 0;
    }

    let mut cost = 0;
    let mut current = cities[0];
    for city in cities.iter().skip(1) {
        cost += costs.get(&(current, city)).expect("Could not find entry");
        current = city;
    }

    cost
}

fn all_routes_costs(routes: &[Route]) -> Vec<i32> {
    let mut costs = HashMap::new();
    let mut cities = HashSet::new();
    for route in routes {
        cities.insert(route.from.as_str());
        cities.insert(route.to.as_str());

        costs.insert((route.from.as_str(), route.to.as_str()), route.cost);
        costs.insert((route.to.as_str(), route.from.as_str()), route.cost);
    }

    let all_routes = permute(cities.into_iter().collect());

    all_routes.into_iter().map(|r| cost(r, &costs)).collect()
}

pub struct Day9 {
    routes: Vec<Route>,
}

impl Solution for Day9 {
    type Fst = i32;
    type Snd = i32;

    fn parse(input: &str) -> Self {
        Day9 {
            routes: parse_routes(input),
        }
    }

    fn fst(&self) -> i32 {
        all_routes_costs(&self.routes)
            .into_iter()
            .min()
            .unwrap_or(0)
    }

    fn snd(&self) -> i32 {
        all_routes_costs(&self.routes)
            .into_iter()
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let day = Day9::parse(INPUT);
        assert_eq!(day.fst(), 141);
        assert_eq!(day.snd(), 736);
    }
}
