pub struct Solution;

use std::collections::{HashSet, HashMap};
impl Solution {
    pub fn calc_equation(eqns: Vec<Vec<String>>, vals: Vec<f64>, qrs: Vec<Vec<String>>) -> Vec<f64> {
        let mut mp = std::collections::HashMap::new();

        let n = eqns.len();
        for i in 0..n {
            let (v1, v2) = (eqns[i][0].clone(), eqns[i][1].clone());

            let v1_en = mp.entry(v1.clone()).or_insert(HashMap::new());
            v1_en.insert(v2.clone(), vals[i]);

            let v2_en = mp.entry(v2).or_insert(HashMap::new());
            v2_en.insert(v1, 1.0 / vals[i]);
        }

        let mut ans = Vec::new();

        for q in qrs {
            let (a, b) = (&q[0], &q[1]);
            let res = match Self::find(&a, &b, &mp, &mut HashSet::new()) {
                None => -1.0,
                Some(v) => v,
            };
            ans.push(res);
        }

        ans
    }

    fn find(a: &str, b: &str, mp: &HashMap<String, HashMap<String, f64>>, visited: &mut HashSet<String>) -> Option<f64> {
        if visited.contains(a) { return None }
        if !mp.contains_key(a) || !mp.contains_key(b) { return None; }
        if a == b { return Some(1.0); }
        if mp[a].contains_key(b) { return Some(mp[a][b]) }

        visited.insert(a.to_owned());
        for (c, &k) in &mp[a] {
            if let Some(u) = Self::find(c, b, mp, visited) {
                return Some(k * u);
            }
        }
        None
    }
}
