use std::cmp::Ordering;

use rand::Rng;

use crate::ray::Ray;

use super::{Aabb, Hit, Hittable};

#[non_exhaustive]
pub enum BvhTree<'a> {
    Leaf {
        bounding_box: Aabb,
        hittable: &'a dyn Hittable,
    },
    Node {
        bounding_box: Aabb,
        left: Box<BvhTree<'a>>,
        right: Box<BvhTree<'a>>,
    },
}

impl<'a> BvhTree<'a> {
    fn new_leaf(hittable: &'a dyn Hittable, time0: f64, time1: f64) -> BvhTree<'a> {
        let Some(bounding_box) = hittable.bounding_box(time0, time1) else {
            panic!("No bounding box")
        };
        BvhTree::Leaf {
            bounding_box,
            hittable,
        }
    }

    pub fn new(hittables: &'a mut [Box<dyn Hittable>], time0: f64, time1: f64) -> BvhTree<'a> {
        let left: BvhTree<'a>;
        let right: BvhTree<'a>;
        let mut rng = rand::thread_rng();
        let axis: usize = rng.gen_range(0..3);
        #[allow(clippy::borrowed_box)]
        let compare = |a: &Box<dyn Hittable>, b: &Box<dyn Hittable>| {
            Self::compare_box(a.as_ref(), b.as_ref(), axis)
        };
        match hittables.len() {
            1 => {
                return BvhTree::new_leaf(hittables[0].as_ref(), time0, time1);
            }
            2 => {
                if compare(&hittables[0], &hittables[1]).is_gt() {
                    left = BvhTree::new_leaf(hittables[0].as_ref(), time0, time1);
                    right = BvhTree::new_leaf(hittables[1].as_ref(), time0, time1);
                } else {
                    left = BvhTree::new_leaf(hittables[1].as_ref(), time0, time1);
                    right = BvhTree::new_leaf(hittables[0].as_ref(), time0, time1);
                }
            }
            _ => {
                hittables.sort_by(compare);
                let (l, r) = hittables.split_at_mut(hittables.len() / 2);
                left = BvhTree::new(l, time0, time1);
                right = BvhTree::new(r, time0, time1);
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

impl Hittable for BvhTree<'_> {
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
