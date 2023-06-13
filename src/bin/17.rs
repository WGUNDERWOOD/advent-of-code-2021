#[derive(Debug)]
struct Target {
    xmin: i32,
    ymin: i32,
    xmax: i32,
    ymax: i32,
}

#[derive(Debug)]
struct Probe {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Probe {
    fn advance(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        if self.vx < 0 {
            self.vx += 1
        };
        if self.vx > 0 {
            self.vx -= 1
        };
        self.vy -= 1;
    }

    fn status(&self, target: &Target) -> Status {
        if self.x >= target.xmin
            && self.x <= target.xmax
            && self.y >= target.ymin
            && self.y <= target.ymax
        {
            Status::Hit
        } else if self.y < target.ymin && self.vy <= 0 {
            Status::Missed
        } else {
            Status::Unknown
        }
    }
}

#[derive(PartialEq, Debug)]
enum Status {
    Unknown,
    Hit,
    Missed,
}

fn parse_input(input: &str) -> Target {
    let split: Vec<&str> = input.split([',', '.', ':', '=', '\n'].as_ref()).collect();
    let xmin = split[2].parse::<i32>().unwrap();
    let ymin = split[6].parse::<i32>().unwrap();
    let xmax = split[4].parse::<i32>().unwrap();
    let ymax = split[8].parse::<i32>().unwrap();
    Target {
        xmin,
        ymin,
        xmax,
        ymax,
    }
}

fn evaluate(probe: &Probe, target: &Target) -> (bool, i32) {
    let mut new_probe = Probe {
        x: probe.x,
        y: probe.y,
        vx: probe.vx,
        vy: probe.vy,
    };
    let mut height = 0;
    while new_probe.status(target) == Status::Unknown {
        new_probe.advance();
        if new_probe.y > height {
            height = new_probe.y
        }
    }
    let hit = new_probe.status(target) == Status::Hit;
    (hit, height)
}

pub fn part_one(input: &str) -> Option<u32> {
    let target = parse_input(input);
    let mut best_height = 0;
    for vx in -30..30 {
        for vy in 0..100 {
            let probe = Probe { x: 0, y: 0, vx, vy };
            let (hit, height) = evaluate(&probe, &target);
            if hit && height > best_height {
                best_height = height;
            }
        }
    }
    Some(best_height as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let target = parse_input(input);
    let mut counter = 0;
    for vx in -400..400 {
        for vy in -100..100 {
            let probe = Probe { x: 0, y: 0, vx, vy };
            let (hit, _height) = evaluate(&probe, &target);
            if hit {
                counter += 1
            }
        }
    }
    Some(counter as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(45));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(112));
    }
}
