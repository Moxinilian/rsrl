use crate::policies::{EnumerablePolicy, Policy};
use rand::{distributions::{Distribution, Uniform}, Rng};

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Random(usize);

impl Random {
    pub fn new(n_actions: usize) -> Self { Random(n_actions) }
}

impl<S> Policy<S> for Random {
    type Action = usize;

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R, _: &S) -> usize {
        Uniform::new(0, self.0).sample(rng)
    }

    fn probability(&self, _: &S, _: &usize) -> f64 { 1.0 / self.0 as f64 }
}

impl<S> EnumerablePolicy<S> for Random {
    fn n_actions(&self) -> usize { self.0 }

    fn probabilities(&self, _: &S) -> Vec<f64> { vec![1.0 / self.0 as f64; self.0].into() }
}

#[cfg(test)]
mod tests {
    use crate::utils::compare_floats;
    use rand::thread_rng;
    use super::{EnumerablePolicy, Policy, Random};

    #[test]
    fn test_sampling() {
        let p = Random::new(2);
        let mut rng = thread_rng();

        let qs = vec![1.0, 0.0];

        let mut n0: f64 = 0.0;
        let mut n1: f64 = 0.0;
        for _ in 0..10000 {
            match p.sample(&mut rng, &qs) {
                0 => n0 += 1.0,
                _ => n1 += 1.0,
            }
        }

        assert!((0.50 - n0 / 10000.0).abs() < 0.05);
        assert!((0.50 - n1 / 10000.0).abs() < 0.05);
    }

    #[test]
    fn test_probabilites() {
        let p = Random::new(4);

        assert!(compare_floats(
            p.probabilities(&vec![1.0, 0.0, 0.0, 1.0]),
            &[0.25; 4], 1e-6
        ));

        let p = Random::new(5);

        assert!(compare_floats(
            p.probabilities(&vec![1.0, 0.0, 0.0, 0.0, 0.0]),
            &[0.2; 5], 1e-6
        ));

        assert!(compare_floats(
            p.probabilities(&vec![0.0, 0.0, 0.0, 0.0, 1.0]),
            &[0.2; 5], 1e-6
        ));
    }
}
