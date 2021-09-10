use rand::distributions::{Distribution, Standard};
use rand::prelude::SliceRandom;
use rand::Rng;

lazy_static!{
    static ref HEALTH_VALS: Vec<u32> = (0..6).map(|n| u32::pow(2,n+1)).collect();
}
#[derive(Debug)]
pub struct BrickMap {
    pub bricks: Vec<Brick>,
}
#[derive(Debug)]
pub struct Brick {
    pub health: u32,
    pub position: (usize, usize)
}
impl Brick {
    pub fn from(point: Point) -> Self {
        Brick { health: point.health.unwrap(), position: (point.x,point.y) }
    }
    pub fn new(health: u32, x: usize, y: usize) -> Self {
        Brick { health, position: (x,y) }
    }
}
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub health: Option<u32>,
}
impl Point {
    pub fn is_brick(&self) -> bool {
        if let Some(_) = self.health {
            true
        }else {
            false
        }
    }
    pub fn is_nearest(&self, current_x: i32, current_y: i32, other: &Point) -> bool {
        
        let dist1 = i32::abs(current_x - self.x as i32)
            + i32::abs(current_y - self.y as i32);
        let dist2 = i32::abs(current_x - other.x as i32)
            + i32::abs(current_y - other.y as i32);
        if dist1 <= dist2 {
            true
        } else {
            false
        }
    }
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let rand_health = match rng.gen::<bool>() {
            true => Some(*HEALTH_VALS.choose(&mut rand::thread_rng()).unwrap_or(&0)),
            false => None,
            
        };
        let (rand_x, rand_y): (usize,usize) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
            health: rand_health,
        }
    }
}