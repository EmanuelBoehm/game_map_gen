use crate::map::{Brick, BrickMap,Point};
use rand::Rng;
use itertools::Itertools;

const CENTER_COUNT: u32 = 50;
pub fn voronoi_map_gen(wh: (usize, usize)) -> BrickMap {

    let mut rng = rand::thread_rng();
    let mut bricks_vec: Vec<Brick> = vec![];

    let mut points: Vec<Point> = (0..CENTER_COUNT).map(|_| rng.gen::<Point>()).collect();
    points.iter_mut().for_each(|point| {
        point.x = point.x % wh.0;
        point.y = point.y % wh.1;
    });
    for (x, y) in (0..wh.0).into_iter().cartesian_product(0..wh.1) {
        let mut closest_point: Option<&Point> = None;
        for point in &points {
            match closest_point {
                None => closest_point = Some(point),
                Some(currently_closest_point) => {
                    if !currently_closest_point.is_nearest(x as i32, y as i32, point) {
                        closest_point = Some(point)
                    }
                }
            }
        }
        if let Some(closest)  = closest_point {
            if closest.is_brick() {
                bricks_vec.push(Brick::new(closest.health.unwrap(), x, y))
            }
        }
    }    
    
    BrickMap { bricks: bricks_vec }
}