use crate::hitable::{surrounding_box, HitRecord, Hitable, AABB};
use crate::random::random_int;
use crate::ray::Ray;

pub struct BVHNode {
    left: Box<dyn Hitable>,
    right: Box<dyn Hitable>,
    aabb: AABB,
}

impl Hitable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.aabb.hit(ray, t_min, t_max) {
            match (
                self.left.hit(ray, t_min, t_max),
                self.right.hit(ray, t_min, t_max),
            ) {
                (Some(left_record), Some(right_record)) => {
                    if left_record.t < right_record.t {
                        Some(left_record)
                    } else {
                        Some(right_record)
                    }
                }
                (Some(left_record), None) => Some(left_record),
                (None, Some(right_record)) => Some(right_record),
                (None, None) => None,
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.aabb)
    }
}

fn box_compare(left: &dyn Hitable, right: &dyn Hitable, i: usize) -> std::cmp::Ordering {
    if let (Some(left_box), Some(right_box)) =
        (left.bounding_box(0.0, 0.0), right.bounding_box(0.0, 0.0))
    {
        left_box.min[i].partial_cmp(&right_box.min[i]).unwrap()
    } else {
        panic!("This should not happen");
    }
}

impl BVHNode {
    pub fn build(mut list: Vec<Box<dyn Hitable>>, t0: f32, t1: f32) -> BVHNode {
        let axis = random_int(0, 3);
        list.sort_unstable_by(|a, b| box_compare(&**a, &**b, axis));

        let n = list.len();
        if n == 1 {
            panic!("should not happen");
        }
        let (left, right): (Box<dyn Hitable>, Box<dyn Hitable>) = match n {
            2 => (list.pop().unwrap(), list.pop().unwrap()),
            3 => (list.pop().unwrap(), Box::new(BVHNode::build(list, t0, t1))),
            _ => (
                Box::new(BVHNode::build(list.drain(..n / 2).collect(), t0, t1)),
                Box::new(BVHNode::build(list, t0, t1)),
            ),
        };
        if let (Some(left_box), Some(right_box)) =
            (left.bounding_box(t0, t1), right.bounding_box(t0, t1))
        {
            BVHNode {
                left,
                right,
                aabb: surrounding_box(left_box, right_box),
            }
        } else {
            panic!("no bounding box in BVHNode::build");
        }
    }
}
