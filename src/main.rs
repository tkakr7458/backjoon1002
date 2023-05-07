use std::io;

#[derive(PartialEq, Eq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        if !(-10_000..10_000).contains(&x) || !(-10_000..10_000).contains(&y) {
            unreachable!("By prob definition")
        }
        Self { x, y }
    }

    fn distance(&self, other: &Self) -> f64 {
        let dx = self.x as f64 - other.x as f64;
        let dy = self.y as f64 - other.y as f64;
        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Circle {
    p: Point,
    radius: i32,
}

enum OverlapStatus {
    // 아예 동일할 때
    Identical,
    // 아예 안 만날 때
    NotMeet,
    // 외접
    Circumscription,
    // 두 점에서 만날 때
    Intersection,
    // 내접
    Inscription,
    // 아예 포함될 때
    Include,
}

impl OverlapStatus {
    fn to_answer(self) -> i32 {
        match self {
            Self::Identical => -1,
            Self::NotMeet => 0,
            Self::Circumscription => 1,
            Self::Intersection => 2,
            Self::Inscription => 1,
            Self::Include => 0,
        }
    }
}

impl Circle {
    fn new(x: i32, y: i32, radius: i32) -> Self {
        if !(0..10_000).contains(&radius) {
            unreachable!("By problem definition")
        }
        Self {
            p: Point::new(x, y),
            radius,
        }
    }

    fn get_overlap_status(&self, other: &Self) -> OverlapStatus {
        if self == other {
            return OverlapStatus::Identical;
        }

        let distance = Point::distance(&self.p, &other.p);
        let radius_subtraction = (self.radius - other.radius).abs();
        if distance == ((self.radius + other.radius) as f64) {
            return OverlapStatus::Circumscription;
        }
        if distance < ((self.radius + other.radius) as f64)
            && ((radius_subtraction as f64) < distance)
        {
            return OverlapStatus::Intersection;
        }
        if distance == radius_subtraction.into() {
            return OverlapStatus::Inscription;
        }
        if ((self.radius + other.radius) as f64) < distance {
            return OverlapStatus::NotMeet;
        }

        return OverlapStatus::Include;
    }
}

struct Turret {
    a: Circle,
    b: Circle,
}

impl Turret {
    fn new(x1: i32, y1: i32, r1: i32, x2: i32, y2: i32, r2: i32) -> Self {
        Self {
            a: Circle::new(x1, y1, r1),
            b: Circle::new(x2, y2, r2),
        }
    }

    fn from_input() -> Turret {
        let numbers = int_input();
        Turret::new(
            numbers[0], numbers[1], numbers[2], numbers[3], numbers[4], numbers[5],
        )
    }

    fn overlap(&self) -> i32 {
        let a = self.a.clone();
        a.get_overlap_status(&self.b).to_answer()
    }
}

fn int_input() -> Vec<i32> {
    let mut a0 = String::new();
    io::stdin().read_line(&mut a0).expect("input error");
    a0.split_whitespace()
        .map(|s| s.trim().parse().expect("parsing error"))
        .collect::<Vec<_>>()
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("input error");
    let n = n.trim().parse().expect("parsing error");

    let mut v = Vec::new();
    for _i in 0..n {
        let turret = Turret::from_input();
        v.push(turret);
    }

    for turret in &v {
        println!("{}", turret.overlap());
    }
}