use std::{collections::HashMap, fs};

use nom::{IResult, character::complete::newline, multi::separated_list1};

use crate::helper::quickfind::QuickFind;

const INPUT_FILE: &str = "src/aoc/y2025/day8.txt";

pub fn day8_fst() -> u32 {
    // parse input
    let content = fs::read_to_string(INPUT_FILE).expect("Failed to read input file");
    let points = parse_points(&content).expect("Failed to parse points").1;

    // compute pairwise distances
    let distances = distances(&points);

    // sort distances starting with smallest
    let mut dist_vec: Vec<((usize, usize), f64)> = distances.into_iter().collect();
    dist_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let dist_vec = dist_vec.iter().take(1000);

    // connect points based on distances
    let mut qf = QuickFind::new(points.len());
    for ((i, j), _) in dist_vec {
        qf.connect(*i, *j);
    }

    let circuit_sizes = qf.group_sizes();
    (circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2]) as u32
}

pub fn day8_snd() -> u64 {
    // parse input
    let content = fs::read_to_string(INPUT_FILE).expect("Failed to read input file");
    let points = parse_points(&content).expect("Failed to parse points").1;

    // compute pairwise distances
    let distances = distances(&points);

    // sort distances starting with smallest
    let mut dist_vec: Vec<((usize, usize), f64)> = distances.into_iter().collect();
    dist_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut qf = QuickFind::new(points.len());
    for ((i, j), _) in dist_vec {
        qf.connect(i, j);
        if qf.groups() == 1 {
            return points[i].0 as u64 * points[j].0 as u64;
        }
    }

    0
}

type Point = (i32, i32, i32);

fn parse_points(content: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(newline, parse_point)(content)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (rest, line) = nom::bytes::complete::take_until("\n")(input)?;
    let mut parts = line.split(',');
    let x = parts.next().unwrap().parse::<i32>().unwrap();
    let y = parts.next().unwrap().parse::<i32>().unwrap();
    let z = parts.next().unwrap().parse::<i32>().unwrap();
    Ok((rest, (x, y, z)))
}

/// Two indexes into the list of points
type PPair = (usize, usize);

fn distances(points: &Vec<Point>) -> HashMap<(usize, usize), f64> {
    let mut distances: HashMap<PPair, f64> = HashMap::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let d = distance(&points[i], &points[j]);
            distances.insert((i, j), d);
        }
    }
    distances
}

fn distance(p1: &Point, p2: &Point) -> f64 {
    let dx = (p1.0 - p2.0) as f64;
    let dy = (p1.1 - p2.1) as f64;
    let dz = (p1.2 - p2.2) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day8_fst(), 175440);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day8_snd(), 3200955921);
    }
}
