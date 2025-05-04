use std::collections::VecDeque;

//BFS algorithm to calculate distances from a starting node to all other nodes in the graph
pub fn bfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<Option<usize>> {
    let mut distance: Vec<Option<usize>> = vec![None; graph.len()]; //initialize distances with None
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(start);
    distance[start] = Some(0);

    while let Some(node) = queue.pop_front() {
        for &neighbor in &graph[node] {
            if distance[neighbor].is_none() {
                distance[neighbor] = Some(distance[node].unwrap() + 1);
                queue.push_back(neighbor);
            }
        }
    }
    distance
}

//function to calculate the mean distance of all the distances calculated from BFS
pub fn mean_distance(distances: &[Vec<Option<usize>>]) -> f64 {
    let mut total_distance = 0;
    let mut total_nodes = 0;

    for distance in distances {
        for d in distance {
            if let Some(dist) = d {
                total_distance += dist;
                total_nodes += 1;
            }
        }
    }

    if total_nodes > 0 {
        total_distance as f64 / total_nodes as f64
    } else {
        0.0
    }
}

//function that utilizes the mean_distance result to calculate the standard deviation of distances
pub fn std_dev(distances: &[Vec<Option<usize>>], mean_distance: f64) -> f64 {
    let mut sum_squared_diff = 0.0;
    let mut total_nodes = 0;

    for distance in distances {
        for d in distance {
            if let Some(dist) = d {
                let diff = (*dist as f64 - mean_distance).powi(2);
                sum_squared_diff += diff;
                total_nodes += 1;
            }
        }
    }

    if total_nodes > 0 {
        let var = sum_squared_diff / total_nodes as f64;
        var.sqrt()
    } else {
        0.0
    }
}

//function that finds the maximum distance of all distances between any two nodes
pub fn max_distance(distances: &[Vec<Option<usize>>]) -> usize {
    let mut max_dist = 0;

    for distance in distances {
        for d in distance {
            if let Some(dist) = d {
                if *dist > max_dist {
                    max_dist = *dist;
                }
            }
        }
    }
    max_dist
}
