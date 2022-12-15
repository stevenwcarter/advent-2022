use std::collections::{HashMap, HashSet};

use advent::read_lines;
use rayon::prelude::*;

fn main() {
    let puzzle_data = "puzzles/15.txt";

    // let answer_a = a(puzzle_data, 2_000_000);
    // println!("A Score: {}", answer_a);

    let b_result = b(puzzle_data, 4_000_000, 4_000_000);
    println!("B Score: {}", b_result);
}

struct SensorReading {
    beacon_x: i64,
    beacon_y: i64,
    sensor_x: i64,
    sensor_y: i64,
}

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
struct Point {
    pub x: i64,
    pub y: i64,
}

struct PointSet {
    pointset: HashMap<i64, HashSet<Point>>,
}

impl PointSet {
    pub fn new(points: Vec<Point>) -> PointSet {
        let mut pointset: HashMap<i64, HashSet<Point>> = HashMap::new();
        for p in points {
            pointset
                .entry(p.y)
                .and_modify(|vec| {
                    vec.insert(p);
                })
                .or_insert({
                    let mut set = HashSet::new();
                    set.insert(p);
                    set
                });
        }
        PointSet { pointset }
    }

    pub fn contains(&self, point: &Point) -> bool {
        if self.pointset.contains_key(&point.y) {
            self.pointset.get(&point.y).unwrap().contains(point)
        } else {
            false
        }
    }

    pub fn get_points_for_y(&self, y: i64) -> Vec<Point> {
        let points = self.pointset.get(&y);
        let mut point_vec: Vec<Point> = Vec::new();

        if let Some(points) = points {
            points.iter().for_each(|p| point_vec.push(p.to_owned()));
        }
        point_vec
    }
}

fn manhattan_distance(sr: &SensorReading) -> u64 {
    sr.sensor_x.abs_diff(sr.beacon_x) + sr.sensor_y.abs_diff(sr.beacon_y)
}

fn get_blocked_points_for_sensor_reading(sr: &SensorReading, y_coord: i64) -> Vec<Point> {
    let distance = manhattan_distance(sr) as i64;
    let mut points: Vec<Point> = Vec::new();

    let min_x = sr.sensor_x - distance;
    let max_x = sr.sensor_x + distance;

    for x in min_x..=max_x {
        let y_offset_range = distance.abs_diff(sr.sensor_x.abs_diff(x) as i64);
        let min_y = sr.sensor_y - y_offset_range as i64;
        let max_y = sr.sensor_y + y_offset_range as i64;
        if y_coord >= min_y && y_coord <= max_y && (y_coord != sr.beacon_y || x != sr.beacon_x) {
            points.push(Point { x, y: y_coord });
        }
    }

    points
}
fn get_blocked_points_for_sensor_reading_b(
    sr: &SensorReading,
    y_coord: i64,
    outer_max_x: i64,
    outer_max_y: i64,
) -> Vec<Point> {
    let distance = manhattan_distance(sr) as i64;
    let mut points: Vec<Point> = Vec::new();

    let mut min_x = sr.sensor_x - distance;
    let mut max_x = sr.sensor_x + distance;
    if min_x < 0 {
        min_x = 0;
    }
    if max_x > outer_max_x {
        max_x = outer_max_x;
    }

    for x in min_x..=max_x {
        let y_offset_range = distance.abs_diff(sr.sensor_x.abs_diff(x) as i64);
        let mut min_y = sr.sensor_y - y_offset_range as i64;
        if min_y < 0 {
            min_y = 0;
        }
        let mut max_y = sr.sensor_y + y_offset_range as i64;
        if max_y > outer_max_y {
            max_y = outer_max_y;
        }
        if y_coord >= min_y && y_coord <= max_y {
            points.push(Point { x, y: y_coord });
        }
    }

    points
}

fn a(path: &str, y_coord: i64) -> usize {
    let sensor_readings: Vec<SensorReading> = read_lines(path)
        .unwrap()
        .iter()
        .map(|l| l.strip_prefix("Sensor at x=").unwrap())
        .map(|l| {
            let parts: Vec<&str> = l.split(": closest beacon is at x=").collect();
            let sensor_coords: Vec<&str> = parts[0].split(", y=").collect();
            let sensor_x = sensor_coords[0].parse::<i64>().unwrap();
            let sensor_y = sensor_coords[1].parse::<i64>().unwrap();

            let beacon_coords: Vec<&str> = parts[1].split(", y=").collect();
            let beacon_x = beacon_coords[0].parse::<i64>().unwrap();
            let beacon_y = beacon_coords[1].parse::<i64>().unwrap();

            SensorReading {
                beacon_x,
                beacon_y,
                sensor_x,
                sensor_y,
            }
        })
        .collect();

    let points: Vec<Point> = sensor_readings
        .iter()
        .flat_map(|sr| get_blocked_points_for_sensor_reading(sr, y_coord))
        .filter(|p| p.y == y_coord)
        .collect();
    let pointset: PointSet = PointSet::new(points);

    pointset.get_points_for_y(y_coord).len()
}

fn b(path: &str, max_x: i64, max_y: i64) -> i64 {
    let sensor_readings: Vec<SensorReading> = read_lines(path)
        .unwrap()
        .iter()
        .map(|l| l.strip_prefix("Sensor at x=").unwrap())
        .map(|l| {
            let parts: Vec<&str> = l.split(": closest beacon is at x=").collect();
            let sensor_coords: Vec<&str> = parts[0].split(", y=").collect();
            let sensor_x = sensor_coords[0].parse::<i64>().unwrap();
            let sensor_y = sensor_coords[1].parse::<i64>().unwrap();

            let beacon_coords: Vec<&str> = parts[1].split(", y=").collect();
            let beacon_x = beacon_coords[0].parse::<i64>().unwrap();
            let beacon_y = beacon_coords[1].parse::<i64>().unwrap();

            SensorReading {
                beacon_x,
                beacon_y,
                sensor_x,
                sensor_y,
            }
        })
        .collect();

    let result: i64 = (0..=max_y)
        .into_par_iter()
        .map(|y| {
            let points: Vec<Point> = sensor_readings
                .iter()
                .flat_map(|sr| get_blocked_points_for_sensor_reading_b(sr, y, max_x, max_y))
                .collect();
            let pointset: PointSet = PointSet::new(points);
            if pointset.get_points_for_y(y).len() < (max_x as usize + 1) {
                // println!("Checking row {}", y);
                for x in 0..max_x {
                    // println!("Checking {},{}", x, y);
                    if !pointset.contains(&Point { x, y }) {
                        return x * 4_000_000 + y;
                    }
                }
            }
            0
        })
        .sum();

    // for y in 0..max_y {
    //     // println!("{}", pointset.get_points_for_y(y).len());
    //     if pointset.get_points_for_y(y).len() < (max_x as usize + 1) {
    //         // println!("Checking row {}", y);
    //         for x in 0..max_x {
    //             // println!("Checking {},{}", x, y);
    //             if !pointset.contains(&Point { x, y }) {
    //                 return x * 4_000_000 + y;
    //             }
    //         }
    //     }
    // }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calcs_manhattan_properly() {
        let sr = SensorReading {
            sensor_x: 8,
            sensor_y: 7,
            beacon_x: 2,
            beacon_y: 10,
        };
        let distance = manhattan_distance(&sr);
        assert_eq!(distance, 9);
    }
    #[test]
    fn it_calcs_blocked_points() {
        let sr = SensorReading {
            sensor_x: 8,
            sensor_y: 7,
            beacon_x: 2,
            beacon_y: 10,
        };
        let points = get_blocked_points_for_sensor_reading(&sr, 10);
        assert_eq!(points.len(), 180);
    }
    #[test]
    fn test_a() {
        let test_path = "test-resources/day15.txt";
        let result = a(test_path, 10);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_b() {
        let test_path = "test-resources/day15.txt";
        let result = b(test_path, 20, 20);
        assert_eq!(result, 56000011);
    }
}
