// use itertools::Itertools;
use ndarray::prelude::*;
// use ndarray::{LinalgScalar, NdFloat};
// use num_traits::{Num, NumOps};
use num_traits::cast::AsPrimitive;
use std::cmp::Ordering;
use crate::numeric_traits::Numeric;


pub fn chi_merge<T: Numeric>(feature: ArrayView1<T>, target: ArrayView1<T>) -> Array1<T> {
    &feature + &target
}


fn unique<T: Numeric>(feature: Array1<T>) -> Array1<T> {
    let mut v = feature.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    v.dedup();
    Array::from(v)
}


fn chi<T>(groups: Array2<T>) -> Array1<f64>
where T: AsPrimitive<f64>,
{
    let l = groups.shape()[0] - 1;
    let mut chi_vec: Vec<f64> = Vec::new();

    for i in 0..l {
        let mut chi: f64 = 0.0;
        let view: Array2<f64> = groups.slice(s![i..i+2, ..]).mapv(|x| x.as_());
        let total: f64 = view.sum();
        let cols: Array1<f64> = view.sum_axis(Axis(0));
        let rows: Array1<f64> = view.sum_axis(Axis(1));

        for j in 0..rows.len() {
            for k in 0..cols.len() {
                let e = rows[j] * cols[k] / total;
                if e != 0.0 {
                    chi += (view[[j, k]] - e).powf(2.0) / e;
                }
            }
        }
        chi_vec.push(chi)
    }

    Array1::from(chi_vec)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_chi_merge() {
        let feat = Array::range(0., 500., 1.);
        let target = Array::ones(500);

        let res = chi_merge(feat.view(), target.view());
        println!("{}", res);
    }

    #[test]
    #[ignore]
    fn test_unique() {
        let target: Array1<f64> = Array::ones(500);
        // let target = array![9,3,2,5,3,5];

        let res = unique(target);
        // let mut res = vec![1,2,3,2,3,5,1];
        // res.dedup();
        println!("{:?}", res);
    }

    #[test]
    // #[ignore]
    fn test_chi() {
        // let target = Array::ones((500, 3));
        let target = array![[0,0,1],
                            [1,0,0],
                            [0,1,0],
                            [0,1,0],
                            [1,1,0]];

        let res = chi(target.mapv(|x| x as u8));
        // let mut res = vec![1,2,3,2,3,5,1];
        // res.dedup();
        println!("{:#?}", res);
    }
}
