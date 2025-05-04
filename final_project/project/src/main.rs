use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{HashSet, HashMap, VecDeque};
mod stats;
use stats::{bfs, mean_distance, std_dev, max_distance};

/// Trip categories: Business or Personal
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Category {
    Business,
    Personal,
}

/// Read (start, stop, category) from CSV or return test cycle
fn read_file(path: &str) -> Vec<(String, String, Category)> {
    if path == "test.txt" {
        return vec![
            ("A".into(), "B".into(), Category::Personal),
            ("B".into(), "C".into(), Category::Personal),
            ("C".into(), "A".into(), Category::Personal),
        ];
    }
    let file = File::open(path).expect("Could not open file");
    let mut rides = Vec::new();
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        let s = line.expect("Error reading line");
        let parts: Vec<&str> = s.trim().split(',').collect();
        // Expecting at least: CATEGORY, START, STOP
        if parts.len() >= 5 {
            let cat = if parts[2] == "Business" {
                Category::Business
            } else {
                Category::Personal
            };
            rides.push((
                parts[3].to_string(),  // START
                parts[4].to_string(),  // STOP
                cat,
            ));
        }
    }

    rides
}

/// Collecting unique nodes
fn unique_nodes(rides: &[(String, String, Category)]) -> HashSet<String> {
    let mut set = HashSet::new();
    for (s, d, _) in rides {
        set.insert(s.clone());
        set.insert(d.clone());
    }
    set
}

/// Build adjacency list and return location index map
fn adjacency_list(
    rides: &[(String, String, Category)],
    nodes: &HashSet<String>
) -> (Vec<Vec<usize>>, Vec<String>) {
    let mut locations: Vec<String> = nodes.iter().cloned().collect();
    locations.sort();
    let mut index = HashMap::new();
    for (i, name) in locations.iter().enumerate() {
        index.insert(name.clone(), i);
    }
    let mut adjacency = vec![Vec::new(); locations.len()];
    for (s, d, _) in rides {
        if let (Some(&u), Some(&v)) = (index.get(s), index.get(d)) {
            adjacency[u].push(v);
        }
    }
    (adjacency, locations)
}

/// Top-N frequent direct routes
fn most_frequent_pairs(
    rides: &[(String, String, Category)],
    most_frequent: usize
) -> Vec<((String, String), usize)> {
    let mut count = HashMap::new();
    for (start, end, _category) in rides {
        let key = (start.clone(), end.clone());
        *count.entry(key).or_insert(0) += 1;
    }
    let mut pairs_counts: Vec<_> = count.into_iter().collect();

    // Sort by descending count, then ascending (start, end) for ties
    pairs_counts.sort_by(|a, b| {
        match b.1.cmp(&a.1) {
            std::cmp::Ordering::Equal => a.0.cmp(&b.0),
            other => other,
        }
    });

    pairs_counts.truncate(most_frequent);
    pairs_counts
}

/// Popular hubs by category
fn popular_hubs(rides: &[(String, String, Category)]) -> (String, String) {
    let mut personal_counts = HashMap::new();
    let mut business_counts = HashMap::new();
    for (start, end, cat) in rides {
        let map = if *cat == Category::Personal { &mut personal_counts } else { &mut business_counts };
        *map.entry(start.clone()).or_insert(0) += 1;
        *map.entry(end.clone()).or_insert(0) += 1;
    }
    let highest_val = |map: &HashMap<String, usize>| {
        map.iter()
            .max_by_key(|&(_, &count)| count)
            .map(|(loc, _)| loc.clone())
            .unwrap_or_default()
    };
    (highest_val(&personal_counts), highest_val(&business_counts))
}

/// Shortest path by hops
fn shortest_path(
    adj: &Vec<Vec<usize>>,
    start: usize,
    end: usize
) -> Option<Vec<usize>> {
    let mut prev = vec![None; adj.len()];
    let mut distance = vec![None; adj.len()];
    let mut queue = VecDeque::new();
    distance[start] = Some(0);
    queue.push_back(start);

    //BFS 
    while let Some(u) = queue.pop_front() {
        if u == end { break; }
        for &v in &adj[u] {
            if distance[v].is_none() {
                distance[v] = Some(distance[u].unwrap() + 1);
                prev[v] = Some(u);
                queue.push_back(v);
            }
        }
    }
    if distance[end].is_none() {
        return None;
    }
    let mut path = Vec::new();
    let mut current = end;
    while let Some(p) = prev[current] {
        path.push(current);
        current = p;
    }
    path.push(start);
    path.reverse();
    Some(path)
}

fn main() {
    // Read and filter rides
    let mut rides = read_file("UberDataset.csv");
    //dropping unknown locations 
    rides.retain(|(s, d, _)| {
        !s.is_empty() && !d.is_empty() &&
        s != "Unknown Location" && d != "Unknown Location"
    });
    println!("Total rides after filter: {}", rides.len());

    //Graph setup
    let nodes = unique_nodes(&rides);
    let (adj, locs) = adjacency_list(&rides, &nodes);

    // Top 5 direct routes
    let top5 = most_frequent_pairs(&rides, 5);
    println!("\nTop 5 routes:");
    for ((from, to), count) in &top5 {
        println!("  {} -> {}: {} trips", from, to, count);
    }

    // 4) Popular locations by category
    let (personal, business) = popular_hubs(&rides);
    println!("\nPersonal: {}\nBusiness: {}", personal, business);

    // 5) Shortest path for the most frequent route
    if let Some(((from, to), _count)) = top5.get(0) {
        if let Some(start_index) = locs.iter().position(|x| x == from) {
            if let Some(end_index) = locs.iter().position(|x| x == to) {
                if let Some(path) = shortest_path(&adj, start_index, end_index) {
                    let names: Vec<&str> = path.iter()
                        .map(|&idx| locs[idx].as_str())
                        .collect();
                    println!("\nShortest {}->{}: {:?}", from, to, names);
                }
            }
        }
    }

    // Graph statistics
    let mut all = Vec::new();
    for i in 0..adj.len() {
        all.push(bfs(&adj, i));
    }
    let mean = mean_distance(&all);
    let standard_deviation = std_dev(&all, mean);
    let max = max_distance(&all);
    println!("\nGraph hops — mean: {:.2}, stddev: {:.2}, max: {}", mean, standard_deviation, max);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_rides() -> Vec<(String,String,Category)> {
        vec![
            ("A".into(), "B".into(), Category::Personal),
            ("A".into(), "B".into(), Category::Personal),
            ("B".into(), "C".into(), Category::Business),
            ("C".into(), "D".into(), Category::Business),
        ]
    }

    #[test]
    fn test_bfs_zeros() {
        let rides = read_file("test.txt");
        let nodes = unique_nodes(&rides);
        let (adj, _) = adjacency_list(&rides, &nodes);
        for i in 0..adj.len() {
            let d = bfs(&adj, i);
            assert_eq!(d[i], Some(0));
        }
    }
    #[test]
    fn test_unique_nodes() {
        let rides = make_rides();
        let nodes = unique_nodes(&rides);
        let mut v: Vec<_> = nodes.into_iter().collect();
        v.sort();
        assert_eq!(v, vec![
            "A".to_string(), "B".to_string(), "C".to_string(),
            "D".to_string() ]);
    }
    #[test]
    fn test_most_frequent_pairs_counts() {
        let rides = make_rides();
        let top2 = most_frequent_pairs(&rides, 2);
        // Expect ("A","B") twice, then the next highest ("B","C") once
        assert_eq!(top2, vec![
            (( "A".to_string(), "B".to_string()), 2),
            (( "B".to_string(), "C".to_string()), 1),
        ]);
    }

    #[test]
    fn test_max_distance() {
        let rides = read_file("test.txt");
        let nodes = unique_nodes(&rides);
        let (adj, _) = adjacency_list(&rides, &nodes);
        let all: Vec<_> = (0..adj.len()).map(|i| bfs(&adj, i)).collect();
        assert_eq!(max_distance(&all), 2);
    }


}
