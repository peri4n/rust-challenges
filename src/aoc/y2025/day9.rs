use std::fs;

use nom::{IResult, character::complete::newline, multi::separated_list1};

const INPUT_FILE: &str = "src/aoc/y2025/day9.txt";

pub fn day9_fst() -> u64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Failed to read input file");
    let points = parse_points(&content).expect("Failed to parse points").1;
    max_rectangle_area(&points)
}

pub fn day9_snd() -> u64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Failed to read input file");
    let points = parse_points(&content).expect("Failed to parse points").1;
    max_rectangle_area_fully_contained(&points)
}

fn max_rectangle_area_fully_contained(points: &[Point]) -> u64 {
    let verticals = build_vertical_segments(points);
    let ys = event_ys(points);
    let mut intervals_by_y: Vec<(i32, Vec<(i32, i32)>)> = Vec::new();
    for &y in &ys {
        intervals_by_y.push((y, intervals_for_y(y, &verticals)));
    }

    let mut max_area = 0u64;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];

            let area = rectangle_area(&(x1, y1), &(x2, y2));

            if area <= max_area {
                continue;
            }

            let x_min = x1.min(x2);
            let x_max = x1.max(x2);
            let y_min = y1.min(y2);
            let y_max = y1.max(y2);

            let mut rows_to_check: Vec<i32> = ys
                .iter()
                .cloned()
                .filter(|yy| *yy >= y_min && *yy <= y_max - 1)
                .collect();

            if y_min == y_max {
                rows_to_check.push(y_min);
            } else {
                rows_to_check.push(y_max - 1);
            }
            rows_to_check.sort_unstable();
            rows_to_check.dedup();

            let mut ok = true;
            for yy in rows_to_check {
                let intervals = intervals_for_y(yy, &verticals);
                let mut covered = false;
                for (l, r) in intervals.iter() {
                    if x_min >= *l && x_max <= *r {
                        covered = true;
                        break;
                    }
                }
                if !covered {
                    ok = false;
                    break;
                }
            }

            if ok {
                if area > max_area {
                    max_area = area;
                }
            }
        }
    }
    max_area
}

#[derive(Clone, Copy)]
struct VSeg {
    x: i32,
    y1: i32,
    y2: i32, // half-open [y1, y2)
}

fn build_vertical_segments(points: &[Point]) -> Vec<VSeg> {
    let mut segs = Vec::new();
    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];
        if p1.0 == p2.0 {
            let (a, b) = if p1.1 <= p2.1 {
                (p1.1, p2.1)
            } else {
                (p2.1, p1.1)
            };
            segs.push(VSeg { x: p1.0, y1: a, y2: b });
        }
    }
    segs
}

fn event_ys(points: &[Point]) -> Vec<i32> {
    let mut ys: Vec<i32> = points.iter().map(|p| p.1).collect();
    ys.sort_unstable();
    ys.dedup();
    ys
}

fn intervals_for_y(y: i32, verticals: &[VSeg]) -> Vec<(i32, i32)> {
    let mut xs: Vec<i32> = Vec::new();
    for s in verticals {
        if y >= s.y1 && y < s.y2 {
            xs.push(s.x);
        }
    }
    xs.sort_unstable();
    xs.dedup();
    let mut intervals: Vec<(i32, i32)> = Vec::new();
    let mut i = 0;
    while i + 1 < xs.len() {
        intervals.push((xs[i], xs[i + 1]));
        i += 2;
    }
    intervals
}

fn max_rectangle_area(points: &[Point]) -> u64 {
    let mut max_area = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let area = rectangle_area(&points[i], &points[j]);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

fn rectangle_area(point1: &Point, point2: &Point) -> u64 {
    let x_diff = 1 + (point1.0 - point2.0).abs() as u64;
    let y_diff = 1 + (point1.1 - point2.1).abs() as u64;
    x_diff * y_diff
}

fn parse_points(content: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(newline, parse_point)(content)
}

type Point = (i32, i32);

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (rest, (x, _, y)) = nom::sequence::tuple((
        nom::character::complete::i32,
        nom::bytes::complete::tag(","),
        nom::character::complete::i32,
    ))(input)?;

    Ok((rest, (x, y)))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution_fst() {
        assert_eq!(day9_fst(), 4739623064);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day9_snd(), 1654141440);
    }
}
