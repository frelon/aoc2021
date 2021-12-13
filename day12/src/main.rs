use std::collections::HashMap;

fn main() {
    let input = input();
    let mut caves = HashMap::new();
    caves.insert("start", vec![]);
    caves.insert("end", vec![]);

    for line in input {
        let mut split = line.split("-");        
        let from = split.next().unwrap();
        let to = split.next().unwrap();

        match caves.get_mut(from) {
            Some(c) => { c.push(to);} ,
            None => {caves.insert(from, vec![to]);},
        }

        match caves.get_mut(to) {
            Some(c) => { c.push(from);} ,
            None => {caves.insert(to, vec![from]);},
        }
    }
    
    println!("CAVES: {:?}", caves);
    let mut paths = vec![];
    visit(&caves, "start", vec![], &mut paths);
    println!("DONE: {:?}", paths.len());
}

fn visit<'a>(caves:&HashMap<&str,Vec<&'a str>>,curr:&'a str,visited:Vec<&'a str>,paths:&mut Vec<Vec<&'a str>>) {
    let mut visited= visited.to_vec();
    visited.push(curr);

    if curr == "end" {
        println!("FIN {:?}", visited);
        paths.push(visited.to_vec());
        return;
    }

    let connections = caves.get(&curr).unwrap();
    
    for connect in connections {
        if connect == &"start" {
            continue;
        }

        if connect.to_uppercase() == connect.to_string() {
            visit(caves, connect, visited.to_vec(), paths);
            continue;
        }

        if !visited.contains(connect) {
            visit(caves, connect, visited.to_vec(), paths);
            continue;
        }

        if !has_visited_twice(&visited) {
            visit(caves, connect, visited.to_vec(), paths);
        }
    }
}

fn has_visited_twice(visited:&Vec<&str>) -> bool {
    for v in visited.iter().filter(|&x| x.to_lowercase() == x.to_string()) {
        if visited.into_iter().filter(|&x| x == v).count() >= 2 {
            return true;
        }
    }

    false
}

fn input() -> Vec<&'static str> {
    // vec![
// "start-A",
// "start-b",
// "A-c",
// "A-b",
// "b-d",
// "A-end",
// "b-end",
    // ]
    // vec![
// "dc-end",
// "HN-start",
// "start-kj",
// "dc-start",
// "dc-HN",
// "LN-dc",
// "HN-end",
// "kj-sa",
// "kj-HN",
// "kj-dc",
    // ] // 19
    vec![
"LP-cb",
"PK-yk",
"bf-end",
"PK-my",
"end-cb",
"BN-yk",
"cd-yk",
"cb-lj",
"yk-bf",
"bf-lj",
"BN-bf",
"PK-cb",
"end-BN",
"my-start",
"LP-yk",
"PK-bf",
"my-BN",
"start-PK",
"yk-EP",
"lj-BN",
"lj-start",
"my-lj",
"bf-LP",
 ]
}
