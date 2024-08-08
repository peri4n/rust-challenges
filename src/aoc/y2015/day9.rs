use std::collections::{HashMap, HashSet};
use std::fs;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, i32};
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated};
use nom::{IResult, Parser};

use crate::helper::combinatorics::permute;

const INPUT_FILE: &str = "src/aoc/y2015/day9.txt";

struct Route<'a> {
    from: &'a str,
    to: &'a str,
    cost: i32,
}

fn parse_route(line: &str) -> IResult<&str, Route<'_>> {
    let locations = terminated(alpha1, tag(" to ")).and(alpha1);
    let to = preceded(tag(" = "), i32);

    locations
        .and(to)
        .map(|((from, to), cost)| Route { from, to, cost })
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

fn all_routes_costs() -> Vec<i32> {
    let definition = fs::read_to_string(INPUT_FILE).expect("Could not read input file.");
    let routes = parse_routes(&definition);

    let mut costs = HashMap::new();
    let mut cities = HashSet::new();
    for route in routes {
        cities.insert(route.from);
        cities.insert(route.to);

        costs.insert((route.from, route.to), route.cost);
        costs.insert((route.to, route.from), route.cost);
    }

    let all_routes = permute(cities.into_iter().collect());

    all_routes.into_iter().map(|r| cost(r, &costs)).collect()
}
pub fn day9_fst() -> i32 {
    all_routes_costs().into_iter().min().unwrap_or(0)
}

pub fn day9_snd() -> i32 {
    all_routes_costs().into_iter().max().unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day9_fst(), 141);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day9_snd(), 736);
    }
}
