use std::cmp::Ordering;

use rand::Rng;

use crate::ray::Ray;

use super::{Aabb, Hit, Hittable};

#[derive(Debug)]
pub struct BvhNode<'a> {
    left: Either<&'a dyn Hittable, Box<BvhNode<'a>>>,
    right: Either<&'a dyn Hittable, Box<BvhNode<'a>>>,
    bounding_box: Aabb,
}

#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<'a> Hittable for Either<&'a dyn Hittable, Box<BvhNode<'a>>> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        match self {
            Self::Left(left) => left.hit(ray, t_min, t_max),
            Self::Right(right) => right.hit(ray, t_min, t_max),
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        match self {
            Self::Left(left) => left.bounding_box(time0, time1),
            Self::Right(right) => right.bounding_box(time0, time1),
        }
    }
}

impl<'a> BvhNode<'a> {
    pub fn new(hittables: &'a mut [Box<dyn Hittable>], time0: f64, time1: f64) -> BvhNode<'a> {
        let left: Either<&'a dyn Hittable, Box<BvhNode<'a>>>;
        let right: Either<&'a dyn Hittable, Box<BvhNode<'a>>>;
        let mut rng = rand::thread_rng();
        let axis: usize = rng.gen_range(0..3);
        #[allow(clippy::borrowed_box)]
        let compare = |a: &Box<dyn Hittable>, b: &Box<dyn Hittable>| {
            Self::compare_box(a.as_ref(), b.as_ref(), axis)
        };
        match hittables.len() {
            1 => {
                left = Either::Left(hittables[0].as_ref());
                right = Either::Left(hittables[0].as_ref());
            }
            2 => {
                if compare(&hittables[0], &hittables[1]).is_gt() {
                    left = Either::Left(hittables[0].as_ref());
                    right = Either::Left(hittables[1].as_ref());
                } else {
                    left = Either::Left(hittables[1].as_ref());
                    right = Either::Left(hittables[0].as_ref());
                }
            }
            _ => {
                hittables.sort_by(compare);
                let (l, r) = hittables.split_at_mut(hittables.len() / 2);
                left = Either::Right(Box::new(BvhNode::new(l, time0, time1)));
                right = Either::Right(Box::new(BvhNode::new(r, time0, time1)));
            }
        }

        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box(time0, time1);
        // FIXME: 一方だけでもあればそれでよいのでは？
        let Some(bounding_box) = box_left.and_then(|box_left| box_right.map(|box_right| box_right.union(&box_left))) else {
            panic!("No bounding box");
        };

        BvhNode {
            left,
            right,
            bounding_box,
        }
    }

    fn compare_box(a: &dyn Hittable, b: &dyn Hittable, axis: usize) -> Ordering {
        let box_a = a.bounding_box(0.0, 0.0);
        let box_b = b.bounding_box(0.0, 0.0);
        let (Some(box_a),Some(box_b)) = (box_a,box_b) else {
            panic!("No bounding box");
        };

        box_a.min[axis].total_cmp(&box_b.min[axis])
    }
}

impl Hittable for BvhNode<'_> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        if !self.bounding_box.hit(&ray, t_min, t_max) {
            return None;
        }

        if let Some(hit_left) = self.left.hit(ray, t_min, t_max) {
            self.right.hit(ray, t_min, hit_left.t).or(Some(hit_left))
        } else {
            self.right.hit(ray, t_min, t_max)
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}
