#![allow(dead_code)]
use advent::read;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Eq, Hash, Clone)]
struct Point {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let puzzle_data = "puzzles/14.txt";

    let answer_a = a(puzzle_data);
    println!("A Score: {}", answer_a);

    let b_result = b(puzzle_data);
    println!("B Score: {}", b_result);
}

struct PointSet {
    pointset: HashMap<i32, Vec<Point>>,
    max_y: i32,
}

impl PointSet {
    pub fn new(points: Vec<Point>) -> PointSet {
        let mut pointset: HashMap<i32, Vec<Point>> = HashMap::new();
        let mut max_y = 0;
        for p in points {
            if p.y > max_y {
                max_y = p.y
            }
            let p_clone = p.clone();
            pointset
                .entry(p.x)
                .and_modify(|vec| vec.push(p))
                .or_insert_with(Vec::new)
                .push(p_clone);
        }
        PointSet { pointset, max_y }
    }
    pub fn insert(&mut self, point: Point) {
        // if point.y > self.max_y {
        //     self.max_y = point.y
        // }
        let p_clone = point.clone();
        self.pointset
            .entry(point.x)
            .and_modify(|vec| vec.push(point))
            .or_insert_with(Vec::new)
            .push(p_clone);
    }

    pub fn contains(&self, point: &Point) -> bool {
        if self.pointset.contains_key(&point.x) {
            self.pointset.get(&point.x).unwrap().contains(point)
        } else {
            false
        }
    }
    pub fn contains_b(&self, point: &Point) -> bool {
        if point.y == (self.max_y() + 2) {
            true
        } else if self.pointset.contains_key(&point.x) {
            self.pointset.get(&point.x).unwrap().contains(point)
        } else {
            point.y == (self.max_y() + 2)
        }
    }

    pub fn get_points_for_x(&self, x: i32) -> Option<&Vec<Point>> {
        self.pointset.get(&x)
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.pointset.len()
    }

    pub fn max_y(&self) -> i32 {
        self.max_y
    }
    pub fn max_x(&self) -> &i32 {
        self.pointset.keys().into_iter().max().unwrap()
    }
    pub fn min_x(&self) -> &i32 {
        self.pointset.keys().into_iter().min().unwrap()
    }
    // pub fn min_y(&self) -> i32 {
    //     self.pointset.iter().map(|p| p.y).min().unwrap()
    // }
}

fn build_points(p1x: i32, p1y: i32, p2x: i32, p2y: i32) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    if p1y < p2y {
        for y in p1y..=p2y {
            points.push(Point { x: p1x, y });
        }
    } else if p1y > p2y {
        for y in p2y..=p1y {
            points.push(Point { x: p1x, y });
        }
    } else if p1x < p2x {
        for x in p1x..=p2x {
            points.push(Point { x, y: p1y });
        }
    } else {
        for x in p2x..=p1x {
            points.push(Point { x, y: p1y });
        }
    }

    points
}

fn find_first_opening_downward(points: &PointSet, x: i32, y: i32) -> (i32, i32) {
    let max_y_points = points.get_points_for_x(x);
    if let Some(max_y_points) = max_y_points {
        let max_y = max_y_points.iter().filter(|p| p.y > y).map(|p| p.y).min();

        if let Some(max_y) = max_y {
            (x, max_y - 1)
        } else {
            (-1, -1)
        }
    } else {
        (-1, -1)
    }
}
fn find_first_opening_downward_b(points: &PointSet, x: i32, y: i32) -> (i32, i32) {
    let max_y_points = points.get_points_for_x(x);
    if let Some(max_y_points) = max_y_points {
        let max_y = max_y_points.iter().filter(|p| p.y > y).map(|p| p.y).min();

        if let Some(max_y) = max_y {
            (x, max_y - 1)
        } else {
            (x, points.max_y() + 2 - 1)
        }
    } else {
        (x, points.max_y() + 2 - 1)
    }
}

fn model_sand(points: &mut PointSet, x: i32, y: i32) -> bool {
    let (newx, newy) = find_first_opening_downward(points, x, y);
    // println!("First opening downward: {},{}", newx, newy);
    if newx == -1 {
        return false;
    }
    let (shiftx, shifty) = if !points.contains(&Point {
        x: newx - 1,
        y: newy + 1,
    }) {
        // println!("Found space left: {},{}", newx - 1, newy + 1);
        (newx - 1, newy + 1)
    } else if !points.contains(&Point {
        x: newx + 1,
        y: newy + 1,
    }) {
        // println!("Found space right: {},{}", newx + 1, newy + 1);
        (newx + 1, newy + 1)
    } else {
        // println!("Sand settled: {},{}\n", newx, newy);
        (newx, newy)
    };

    if shiftx != newx || shifty != newy {
        model_sand(points, shiftx, shifty)
    } else {
        points.insert(Point { x: newx, y: newy });

        true
    }
}
fn model_sand_b(points: &mut PointSet, x: i32, y: i32) -> bool {
    let (newx, newy) = find_first_opening_downward_b(points, x, y);
    if newx == -1 {
        println!("Equaled -1");
        return false;
    }
    let (shiftx, shifty) = if !points.contains_b(&Point {
        x: newx - 1,
        y: newy + 1,
    }) {
        (newx - 1, newy + 1)
    } else if !points.contains_b(&Point {
        x: newx + 1,
        y: newy + 1,
    }) {
        (newx + 1, newy + 1)
    } else {
        (newx, newy)
    };

    if shiftx != newx || shifty != newy {
        model_sand(points, shiftx, shifty)
    } else {
        points.insert(Point { x: newx, y: newy });

        true
    }
}

fn build_pointset(path: &str) -> PointSet {
    let points: Vec<Point> = read(path)
        .lines()
        .flat_map(|l| {
            let mut inner_points_iter = l
                .split(" -> ")
                .map(|pts| {
                    let pt_set: Vec<i32> = pts
                        .split(',')
                        .map(|pt| pt.parse::<i32>().unwrap())
                        .collect();

                    pt_set
                })
                .peekable();

            let mut inner_points: Vec<Point> = Vec::new();
            let mut old = inner_points_iter.next().unwrap();
            while inner_points_iter.peek().is_some() {
                let p1 = old;
                let p2 = inner_points_iter.next().unwrap();

                build_points(p1[0], p1[1], p2[0], p2[1])
                    .iter()
                    .for_each(|p| {
                        inner_points.push(Point { x: p.x, y: p.y });
                    });

                old = p2;
            }

            inner_points
        })
        .collect();

    PointSet::new(points)
}

fn a(path: &str) -> i32 {
    let mut pointset = build_pointset(path);

    let mut placed = true;
    let mut sand_counter = -1;
    while placed {
        sand_counter += 1;
        placed = model_sand(&mut pointset, 500, 0);
    }

    sand_counter
}

fn add_floor(points: &mut PointSet) {
    let maxy = points.max_y();
    let minx = points.min_x();
    let maxx = points.max_x();
    let delta = maxx - minx;

    let y = maxy + 2;
    build_points(minx - (4 * delta), y, maxx + (4 * delta), y)
        .iter()
        .for_each(|p| {
            points.insert(Point { x: p.x, y: p.y });
        });
}

fn b(path: &str) -> i32 {
    let mut pointset = build_pointset(path);
    add_floor(&mut pointset);

    let mut placed = true;
    let mut sand_counter = -1;
    let end_point = Point { x: 500, y: 0 };
    while placed && !pointset.contains(&end_point) {
        sand_counter += 1;
        placed = model_sand_b(&mut pointset, 500, 0);
    }

    sand_counter + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_pointset() {
        let test_path = "test-resources/day14.txt";
        let result = build_pointset(test_path);
        assert_eq!(result.len(), 20);
    }
    #[test]
    fn test_a() {
        let test_path = "test-resources/day14.txt";
        let result = a(test_path);
        assert_eq!(result, 24);
    }

    #[test]
    fn test_b() {
        let test_path = "test-resources/day14.txt";
        let result = b(test_path);
        assert_eq!(result, 93);
    }

    #[test]
    fn test_b_actual() {
        let test_path = "puzzles/14.txt";
        let result = b(test_path);
        assert_eq!(result, 25771);
    }
}
