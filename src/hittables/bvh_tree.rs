use std::cmp::Ordering;

use rand::Rng;

use crate::ray::Ray;

use super::{Aabb, Hit, Hittable, HittableVec};

#[non_exhaustive]
pub enum BvhTree {
    Leaf {
        bounding_box: Aabb,
        hittable: Box<dyn Hittable>,
    },
    Node {
        bounding_box: Aabb,
        left: Box<BvhTree>,
        right: Box<BvhTree>,
    },
}

impl BvhTree {
    fn new_leaf(hittable: Box<dyn Hittable>, time0: f64, time1: f64) -> BvhTree {
        let Some(bounding_box) = hittable.bounding_box(time0, time1) else {
            panic!("No bounding box")
        };
        BvhTree::Leaf {
            bounding_box,
            hittable,
        }
    }

    pub fn new(mut hittables: HittableVec, time0: f64, time1: f64) -> BvhTree {
        let left: BvhTree;
        let right: BvhTree;
        let mut rng = rand::thread_rng();
        let axis: usize = rng.gen_range(0..3);
        match hittables.len() {
            1 => {
                return BvhTree::new_leaf(hittables.pop().unwrap(), time0, time1);
            }
            2 => {
                let r = hittables.pop().unwrap();
                let l = hittables.pop().unwrap();
                if Self::compare_box(l.as_ref(), r.as_ref(), axis).is_gt() {
                    left = BvhTree::new_leaf(l, time0, time1);
                    right = BvhTree::new_leaf(r, time0, time1);
                } else {
                    left = BvhTree::new_leaf(r, time0, time1);
                    right = BvhTree::new_leaf(l, time0, time1);
                }
            }
            _ => {
                hittables.sort_by(|a, b| Self::compare_box(a.as_ref(), b.as_ref(), axis));
                let hittables_right = hittables.split_off(hittables.len() / 2);
                left = BvhTree::new(hittables, time0, time1);
                right = BvhTree::new(hittables_right, time0, time1);
            }
        }

        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box(time0, time1);
        // FIXME: 一方だけでもあればそれでよいのでは？
        let Some(bounding_box) = box_left.and_then(|box_left| box_right.map(|box_right| box_right.union(&box_left))) else {
            panic!("No bounding box");
        };

        BvhTree::Node {
            bounding_box,
            left: Box::new(left),
            right: Box::new(right),
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

impl Hittable for BvhTree {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        if !self.bounding_box(0.0, 0.0).unwrap().hit(ray, t_min, t_max) {
            return None;
        }

        match self {
            BvhTree::Leaf { hittable, .. } => hittable.hit(ray, t_min, t_max),
            BvhTree::Node { left, right, .. } => {
                if let Some(hit_left) = left.hit(ray, t_min, t_max) {
                    right.hit(ray, t_min, hit_left.t).or(Some(hit_left))
                } else {
                    right.hit(ray, t_min, t_max)
                }
            }
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(match self {
            BvhTree::Leaf { bounding_box, .. } => *bounding_box,
            BvhTree::Node { bounding_box, .. } => *bounding_box,
        })
    }
}
