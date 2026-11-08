use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
struct Santa {
    name: String,
    clan: String,
    email_address: String,
    history: Option<Vec<Option<String>>>,
}

/// Read csv file as polars DataFrame
///
/// # Arguments
///
/// * `filename` - Path to read.
///
/// # Returns
///
/// * `HashMap<String, Santa>` - Map of name and Sata``
fn read_family_csv(santas: &mut HashMap<String, Santa>, filename: &PathBuf)  {
    

    
    let text = fs::read_to_string(filename.join("family.csv")).unwrap();
    let mut lines = text.split("\n");
    let header: Vec<&str> = lines.next().unwrap().split(",").collect();

    let name_idx = header.binary_search(&"name").unwrap();
    let group_idx = header.binary_search(&"group").unwrap();
    let email_idx = header.binary_search(&"email").unwrap();

    for line in lines {
        let row: Vec<&str> = line.split(",").collect();
        let name = row[name_idx].to_string();
        let santa = Santa {
            name: name.to_string(),
            clan: row[group_idx].to_string(),
            email_address: row[email_idx].to_string(),
            history: None,
        };
        santas.insert(name, santa);
    }

}

fn set_history(santas: &mut HashMap<String, Santa>, filename: &PathBuf) {
    let text = fs::read_to_string(filename.join("history.csv")).unwrap();
    let lines: Vec<&str> = text.split("\n").collect();

    let header: Vec<&str> = lines.first().unwrap().split(",").collect();

    for idx in 0..header.len() {
        let name = header[idx];
        if name == "" {
            continue;
        }

        if !santas.contains_key(name) {
            panic!("History contains unknown name {}", name);
        }
        let santa = santas.get_mut(name).unwrap();

        let mut column: Vec<Option<String>> = vec![];
        for row_idx in 1..lines.len() {
            let row: Vec<&str> = lines[row_idx].split(",").collect();
            let value = if row[idx] == "" {
                None
            } else {
                Some(row[idx].to_string())
            };
            column.push(value.clone());
        }
        // contents.insert(name.to_string(), column);

        santa.history = Some(column);
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
