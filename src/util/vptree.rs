/* 
 * Implements a (randomized, fixed-seed) vantage-point tree for fast nearest 
 * neighbor calculations, will panic if queried when empty. The `build` 
 * constructor always returns a boxed VPTree. This will take ownership of the data.
 * */

use num::Float;
use rand::{SeedableRng, RngCore};

pub trait MetricPoint {
    type Dist: Float;
    fn dist(from: &Self, to: &Self) -> Self::Dist;
}

#[derive(Debug)]
pub struct VPTree<D: Float, T: MetricPoint<Dist = D>> {
    root: T,
    threshold: D,
    near: Option<Box<VPTree<D, T>>>,
    far: Option<Box<VPTree<D, T>>>,
    pub height: usize,
}

fn to_optref<T>(opt: &Option<Box<T>>) -> Option<&T> {
    opt.as_ref().map(|b| b.as_ref())
}

fn split_at<T>(mut vec: Vec<T>, idx: usize) -> (Vec<T>, Vec<T>) {
    let right = vec.split_off(idx);
    (vec, right)
}

impl<D: Float, T: MetricPoint<Dist = D>> VPTree<D, T> {
    pub fn build(points: Vec<T>) -> Self {
        *Self::construct(points).unwrap()
    }

    fn construct(mut points: Vec<T>) -> Option<Box<Self>> {
        if points.is_empty() {
            return None;
        }

        let n = points.len();

        let mut rng = rand::rngs::SmallRng::seed_from_u64(0u64);
        
        let root_idx = (rng.next_u32() as usize) % n;
        points.swap(0, root_idx);

        let root = points.swap_remove(0);

        if n == 1 {
            return Some(Box::new(
                VPTree{
                    root, 
                    threshold: num::zero(), 
                    near: None, 
                    far: None,
                    height: 0
                }
            ));
        }       

        let split_idx = (rng.next_u32() as usize) % (n - 1);
        let threshold = MetricPoint::dist(&root, &points[split_idx]);

        let mut pos = 0;

        for i in 0..(n - 1) {
            if MetricPoint::dist(&root, &points[i]) <= threshold {
                points.swap(pos, i);
                pos += 1
            }
        }

        let (left, right) = split_at(points, pos);
            
        let near = VPTree::construct(left);
        let far = VPTree::construct(right);
            
        
        let height = {
            let mut max = 0;
            if let Some(near) = &near {
                max = near.height
            }
            if let Some(far) = &far {
                max = max.max(far.height)
            }
            max
        } + 1;

        Some(Box::new(
            VPTree{
                root,
                threshold,
                near,
                far,
                height,
            }
        ))
    }

    pub fn nearest<'a, 'b>(&'a self, point: &'b T) -> &'a T {
        Self::nearest_impl(Some(self), point).unwrap()
    }

    fn nearest_impl<'a, 'b>(vptree: Option<&'a Self>, point: &'b T) -> Option<&'a T> {
        vptree?;

        let vpt = vptree.unwrap();

        let root = &vpt.root;
        let root_dist = MetricPoint::dist(root, point);

        if root_dist <= vpt.threshold {
            let mut closest =
                Self::get_closest(point, Some(root), Self::nearest_impl(to_optref(&vpt.near), point));
            let closest_dist = MetricPoint::dist(closest.unwrap(), point); // this is guaranteed to
                                                                           // unwrap
            if closest_dist > vpt.threshold - root_dist {
                closest = 
                    Self::get_closest(point, closest, Self::nearest_impl(to_optref(&vpt.far), point));
            }
            closest
        } else {
            let mut closest = 
                Self::get_closest(point, Some(root), Self::nearest_impl(to_optref(&vpt.far), point));
            let closest_dist = MetricPoint::dist(closest.unwrap(), point); // this is guaranteed to
                                                                           // unwrap
            if closest_dist > root_dist - vpt.threshold {
                closest = 
                    Self::get_closest(point, closest, Self::nearest_impl(to_optref(&vpt.near), point));
            }
            closest
        } 
    }


    fn get_closest<'a, 'b>(point: &'b T, a: Option<&'a T>, b: Option<&'a T>) -> Option<&'a T> {
        if a.is_none() {
            return b
        }
        if b.is_none() {
            return a
        }
        if MetricPoint::dist(a.unwrap(), point) <= MetricPoint::dist(b.unwrap(), point) {
            a
        } else {
            b
        }
    }
}

/***************** tests ***********************/

#[cfg(test)]
mod vptree_tests {
    
    use crate::util::vptree::{VPTree, MetricPoint};

    impl MetricPoint for [f64; 2] {
        type Dist = f64;

        fn dist(from: &Self, to: &Self) -> Self::Dist {
            ((from[0] - to[0])*(from[0] - to[0]) + (from[1] - to[1])*(from[1] - to[1])).sqrt()
        }
    }

    #[test]
    fn vptree_one() {
        let data = [[1., 2.], [2., 1.], [0., 0.], [-1., 3.]];
        let vp = VPTree::build(data.to_vec());
        assert_eq!(vp.nearest(&[4.0, 3.0]), &[2., 1.]);
    }
}
