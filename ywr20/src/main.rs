// FILL in the blanks and FIX the errors
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58);

    // Get returns an Option<&V>
    let score = scores.get("Sunface");
    assert_eq!(score, Some(&98));

    if scores.contains_key("Daniel") {
        // Indexing returns a value V
        let score = scores["Daniel"];
        assert_eq!(score, 95);
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in scores {
        println!("The score of {} is {}", name, score);
    }
////////////////////////////////////////////////////////////////

let teams: [(&str, i32); 3] = [
    ("Chinese Team", 100),
    ("American Team", 10),
    ("France Team", 50),
];

let mut teams_map1 = HashMap::new();
for team in &teams {
    teams_map1.insert(team.0, team.1);
}

let teams_map2: HashMap<&str, i32> = teams.iter().cloned().collect();

// Print the 'teams' array
println!("Teams:");
for (name, score) in &teams {
    println!("{}: {}", name, score);
}

// Print the 'teams_map1' HashMap
println!("\nTeams Map 1:");
for (name, score) in teams_map1.iter() {
    println!("{}: {}", name, score);
}

// Print the 'teams_map2' HashMap
println!("\nTeams Map 2:");
for (name, score) in teams_map2.iter() {
    println!("{}: {}", name, score);
}
}


