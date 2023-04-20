mod solution;
pub use solution::*;

pub fn arr_to_vec<const N: usize, T>(arr: [T; N]) -> Vec<T> {
    arr.into()
}

pub fn arr_to_vec_by<const N: usize, T, J>(arr: [T; N], f: impl Fn(T) -> J) -> Vec<J> {
    arr.into_iter().map(f).collect()
}

pub fn arr_to_vec_2d<const N: usize, const M: usize, T>(array: [[T; N]; M]) -> Vec<Vec<T>> {
    let mut vec = Vec::new();
    for row in array.into_iter() {
        vec.push(row.into());
    }
    vec
}

pub fn arr_to_vec_2d_by<const N: usize, const M: usize, T, J>(array: [[T; N]; M], f: impl Fn(T) -> J) -> Vec<Vec<J>> {
    let mut vec = Vec::new();
    for row in array.into_iter() {
        vec.push(row.into_iter().map(&f).collect());
    }
    vec
}


pub fn adj_list(n: usize, edges: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    let mut adj = vec![Vec::new(); n];
    edges.into_iter().for_each(|v| {
        let a = v[0] as usize;
        let b = v[1] as usize;
        adj[a].push(b);
        adj[b].push(a);
    });
    adj
}

pub fn adj_list_dir(n: usize, edges: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    let mut adj = vec![Vec::new(); n];
    edges.into_iter().for_each(|v| {
        adj[v[0] as usize].push(v[1] as usize);
    });
    adj
}
