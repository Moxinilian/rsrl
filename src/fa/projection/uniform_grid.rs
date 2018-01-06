use super::{Projector, Projection};
use geometry::{Space, RegularSpace};
use geometry::dimensions::{Dimension, Continuous, Partitioned};
use ndarray::Array1;


#[derive(Serialize, Deserialize)]
pub struct UniformGrid {
    n_features: usize,
    input_space: RegularSpace<Partitioned>,
}

impl UniformGrid {
    pub fn new(input_space: RegularSpace<Partitioned>) -> Self {
        let n_features = input_space.span().into();

        UniformGrid {
            n_features: n_features,
            input_space: input_space,
        }
    }

    fn hash(&self, input: &[f64]) -> usize {
        let mut in_it = input.iter().rev();
        let mut d_it = self.input_space.iter().rev();

        let acc = d_it.next().unwrap().convert(*in_it.next().unwrap());

        d_it.zip(in_it)
            .fold(acc, |acc, (d, v)| d.convert(*v) + d.density() * acc)
    }
}

impl Projector<RegularSpace<Continuous>> for UniformGrid {
    fn project(&self, input: &Vec<f64>) -> Projection {
        Projection::Sparse(Array1::from_vec(vec![self.hash(input)]))
    }

    fn dim(&self) -> usize {
        self.input_space.dim()
    }

    fn size(&self) -> usize {
        self.n_features
    }

    fn activation(&self) -> usize {
        1
    }

    fn equivalent(&self, other: &Self) -> bool {
        self.dim() == other.dim() && self.size() == other.size()
    }
}


#[cfg(test)]
mod tests {
    use super::{Projector, UniformGrid};
    use geometry::RegularSpace;
    use geometry::dimensions::Partitioned;


    #[test]
    fn test_1d() {
        let ds = RegularSpace::new(vec![Partitioned::new(0.0, 10.0, 10)]);
        let t = UniformGrid::new(ds);

        assert_eq!(t.size(), 10);

        for i in 0..10 {
            let out = t.project(&vec![i as u32 as f64]).to_vec();

            let mut expected = vec![0.0; 10];
            expected[i] = 1.0;

            assert_eq!(out, expected);
        }
    }

    #[test]
    fn test_2d() {
        let ds = RegularSpace::new(vec![Partitioned::new(0.0, 10.0, 10); 2]);
        let t = UniformGrid::new(ds);

        assert_eq!(t.size(), 100);

        for i in 0..10 {
            for j in 0..10 {
                let out = t.project(&vec![i as u32 as f64, j as u32 as f64]).to_vec();

                let mut expected = vec![0.0; 100];
                expected[j * 10 + i] = 1.0;

                assert_eq!(out, expected);
            }
        }
    }

    #[test]
    fn test_3d() {
        let ds = RegularSpace::new(vec![Partitioned::new(0.0, 10.0, 10); 3]);
        let t = UniformGrid::new(ds);

        assert_eq!(t.size(), 1000);

        for i in 0..10 {
            for j in 0..10 {
                for k in 0..10 {
                    let out = t.project(&vec![i as u32 as f64, j as u32 as f64, k as u32 as f64])
                        .to_vec();

                    let mut expected = vec![0.0; 1000];
                    expected[k * 100 + j * 10 + i] = 1.0;

                    assert_eq!(out, expected);
                }
            }
        }
    }
}
