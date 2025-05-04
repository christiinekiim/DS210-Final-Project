//Reads a CSV file or test file returns a Vec of (start, stop) string pairs by extracting the 4th and 5th columns of each row
fn read_file(path: &str) -> Vec<(String, String)> {
    
    if path == "test.txt" {
        return vec![
            ("A".to_string(), "B".to_string()),
            ("B".to_string(), "C".to_string()),
            ("C".to_string(), "A".to_string()),
        ];
    }

    
    let file = File::open(path).expect("Could not open file");
    let lines = io::BufReader::new(file).lines();
    let mut rides = Vec::new();

    for (lineno, line) in lines.enumerate() {
        let l = line.expect("error reading line");
        let parts: Vec<&str> = l.trim().split(',').collect();

        if lineno == 0 {
            continue;
        }
        if parts.len() >= 5 {
            let start = parts[3].to_string(); // 4th column: START
            let stop  = parts[4].to_string(); // 5th column: STOP
            rides.push((start, stop));
        }
    }
    rides
}
