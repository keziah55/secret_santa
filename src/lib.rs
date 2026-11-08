use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
struct Santa {
    name: String,
    clan: String,
    email_address: String,
    history: Vec<Option<String>>,
}

/// Read csv file and update `santas` hashmap
///
/// # Arguments
///
/// * `santas` - Mutable reference to hashmap
/// * `filename` - Path to read
fn read_family_csv(santas: &mut HashMap<String, Santa>, filename: &PathBuf) {
    let text = fs::read_to_string(filename.join("family.csv")).unwrap();
    let mut lines = text.split("\n");
    let header: Vec<&str> = lines.next().unwrap().split(",").collect();

    let name_idx = header.iter().position(|&s| s == "name").unwrap();
    let group_idx = header.iter().position(|&s| s == "group").unwrap();
    let email_idx = header.iter().position(|&s| s == "email").unwrap();

    for line in lines {
        let row: Vec<&str> = line.split(",").collect();
        let name = row[name_idx].to_string();
        let santa = Santa {
            name: name.to_string(),
            clan: row[group_idx].to_string(),
            email_address: row[email_idx].to_string(),
            history: vec![],
        };
        santas.insert(name, santa);
    }
}

/// Read `history.csv` and update `santas` hashmap
///
/// * `santas` - Mutable reference to hashmap
/// * `filename` - Path to read
fn set_history(santas: &mut HashMap<String, Santa>, filename: &PathBuf) {
    let text = fs::read_to_string(filename.join("history.csv")).unwrap();
    let lines: Vec<&str> = text.split("\n").collect();

    // get header row from `lines`
    let header: Vec<&str> = lines.first().unwrap().split(",").collect();

    // iterate over columns of data
    for idx in 0..header.len() {
        let name = header[idx];
        if name.trim().is_empty() {
            continue;
        }

        if !santas.contains_key(name) {
            panic!("History contains unknown name {}", name);
        }
        let santa = santas.get_mut(name).unwrap();

        for row_idx in 1..lines.len() {
            if lines[row_idx].trim().is_empty() {
                continue;
            }

            let row: Vec<&str> = lines[row_idx].split(",").collect();
            let value = if row[idx] == "" {
                None
            } else {
                Some(row[idx].to_string())
            };
            santa.history.push(value.clone());
        }
    }
}

pub fn generate(data_path: &PathBuf) {
    let mut santas: HashMap<String, Santa> = HashMap::new();
    read_family_csv(&mut santas, data_path);
    set_history(&mut santas, data_path);

    for santa in santas {
        println!("{:#?}", santa);
    }
}

// TESTS
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rstest::rstest;
//     #[test]
//     fn test_get_santa_data() {
//         let data_path = Path();
//     }
// }
