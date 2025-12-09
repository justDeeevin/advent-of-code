use std::collections::BinaryHeap;

use geo::{Area, Contains, Coord, GeoFloat, LineString, Polygon, Rect};

const INPUT: &str = include_str!("../../input.txt");
// const INPUT: &str = r#"7,1
// 11,1
// 11,7
// 9,7
// 9,5
// 2,5
// 2,3
// 7,3"#;

fn main() {
    let points = INPUT.lines().map(parse_coord).collect::<Vec<_>>();

    let full_polygon = Polygon::new(points.iter().copied().collect::<LineString<_>>(), vec![]);

    let mut rects = points
        .iter()
        .enumerate()
        .flat_map(|(i, p)| {
            points
                .iter()
                .skip(i + 1)
                .map(move |q| ByArea(Rect::new(*p, *q)))
        })
        .filter(|ByArea(r)| full_polygon.contains(r))
        .collect::<BinaryHeap<_>>();

    let out = true_area(rects.pop().unwrap().0);

    dbg!(out);
}

struct ByArea(Rect);

impl PartialEq for ByArea {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for ByArea {}

impl PartialOrd for ByArea {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ByArea {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.unsigned_area().total_cmp(&other.0.unsigned_area())
    }
}

fn parse_coord(s: &str) -> Coord {
    let (x, y) = s.split_once(',').unwrap();
    Coord {
        x: x.parse().unwrap(),
        y: y.parse().unwrap(),
    }
}

fn true_area<T: GeoFloat>(r: Rect<T>) -> T {
    ((r.max().x - r.min().x).abs() + T::one()) * ((r.max().y - r.min().y).abs() + T::one())
}
