// fn get_anagrams(dico: Vec<&str>) -> Vec<String> {
//     dico.iter().map(|n: &str| 1).collect()
// }
use std::collections::HashMap;
fn sort_str(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    let mut result = String::new();
    for c in chars {
        result.push(c);
    }
    result
}

fn group_by_key(v: Vec<(String, &str)>) -> HashMap<String, Vec<&str>> {
    let mut map = HashMap::new();
    for (a, b) in v {
        if !map.contains_key(&a) {
            let mut all_values:Vec<&str> = Vec::new();
            all_values.push(b);
            map.insert(a, all_values);
        } else {
            if let Some(boxed) = map.get_mut(&a) {
                (*boxed).push(b);
            }
        }
    }
    map
}

fn print_only_anagrams(map: HashMap<String, Vec<&str>>) {
    for (_, val) in map.iter() {
        if val.len() >= 2 {
            for c in val {
                print!("{}, ", c);
            }
            println!("");
        }
    }
}

fn main() {
    let dico = vec!["debitcard", "elvis", "silent", "badcredit", "lives", "freedom", "listen", "levis", "money"];
    let v: Vec<(String, &str)> = dico.into_iter().map(|x| (sort_str(x), x)).collect();
    let map = group_by_key(v);
    print_only_anagrams(map);
}
