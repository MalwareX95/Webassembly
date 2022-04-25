use std::{collections::HashSet, time::Duration};
use wasm_bindgen::prelude::*;
use async_std::{task};
use futures::{join};

pub fn get_connected_nodes(
    node: usize,
    graph: &[f32],
    n_size: usize) -> Vec<(usize, f32)> {
    let linear_index = node * n_size;

    let result: Vec<(usize, f32)> = graph[linear_index..linear_index + n_size]
        .iter()
        .enumerate()
        .filter(|(_, &weight)| weight != f32::INFINITY)
        .map(|(n, &weight)| (n, weight))
        .collect();
    
    result
}

fn min_distance(nodes: &Vec<(usize, f32)>) -> Option<&(usize, f32)> {
    if nodes.is_empty() {
        return None;
    }

    let mut min = &nodes[0];

    for node in nodes {
        if min.1 > node.1 {
            min = node
        }
    }

    Some(min)
}

async fn partialSum(start: usize, n: usize) -> i32 {
    let mut sum = 0i32;
    for n in (start..1000).step_by(n)  {
        sum += n as i32;
        web_sys::console::log_1(&format!("Thread: {start}, sum: {sum}").into())
    }
    sum
}

#[wasm_bindgen]
pub async fn parrallelSum() -> i32  {
    let a = partialSum(0, 2);
    let b = partialSum(1, 2);
    let (aResult, bResult) = join!(a, b);
    aResult + bResult
}


#[wasm_bindgen]
pub async fn hello_delay(word: String) -> String {
    task::sleep(Duration::from_millis(1000)).await;
    format!("Hello {word}").to_string()
}

#[wasm_bindgen]
pub fn dijkstra(
    start_node: usize, 
    n_size: usize, 
    graph: &[f32]) -> Vec<i32>
{
    let mut q: HashSet<usize> = HashSet::from_iter(0..n_size);
    let mut s: HashSet<usize> = HashSet::with_capacity(n_size);
    
    let mut p: Vec<i32> = 
        (0..n_size)
        .map(|_| -> i32 { -1 })
        .collect();

    let mut d: Vec<f32> = 
        (0..n_size)
        .map(|_| -> f32 { std::f32::INFINITY })
        .collect();
        
    d[start_node] = 0f32;

    for _ in 0..n_size {
        let unprocessed_nodes: Vec<(usize, f32)> = q
            .iter()
            .map(|&n| (n, d[n]))
            .collect();
        
        
        let &(u, _) = min_distance(&unprocessed_nodes).unwrap();
        q.remove(&u);
        s.insert(u);

        for (w, weight) in get_connected_nodes(u, graph, n_size) {
            let sum_weight = d[u] + weight;

            if q.contains(&w) && d[w] > sum_weight  {
                d[w] = sum_weight;
                p[w] = u as i32;
            }
        }
    }

    return p;
}