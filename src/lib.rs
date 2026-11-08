use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, string};

#[derive(Debug)]
struct Santa {
    name: String,
    clan: String,
    email_address: String,
    history: Vec<Option<String>>,
}

type SantaMap = HashMap<String, Santa>;
type Connections = HashMap<String, Vec<String>>;

/// Read csv file and update `santas` hashmap
///
/// # Arguments
///
/// * `santas` - Mutable reference to hashmap
/// * `filename` - Path to read
fn read_family_csv(santas: &mut SantaMap, filename: &PathBuf) {
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
fn set_history(santas: &mut SantaMap, filename: &PathBuf) {
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

        for row_idx in lines.len()..1 {
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

fn set_possible_recipients(
    santa_name: &String,
    santas: &SantaMap,
    exclude_history: Option<usize>,
) -> Vec<String> {
    let exclude_size = match exclude_history {
        None => 0,
        Some(value) => value,
    };

    let mut recips = vec![];

    let santa = &santas[santa_name];

    for key in santas.keys() {
        if *key == santa.name {
            continue;
        }

        let person = &santas[key];

        if person.clan == santa.clan {
            continue;
        }

        let mut santa_in_history: bool = false;
        if exclude_size > 0 {
            for (idx, item) in person.history.iter().enumerate() {
                if idx < exclude_size && item.is_some() {
                    if item.as_deref().unwrap() == santa.name {
                        santa_in_history = true;
                        break;
                    }
                }
            }
        }
        if santa_in_history {
            continue;
        }
        recips.push(key.clone());
    }

    return recips;
}

fn make_santas(data_path: &PathBuf) -> (SantaMap, Connections) {
    let mut santas: SantaMap = HashMap::new();
    let mut possible_recipients: Connections = HashMap::new();

    read_family_csv(&mut santas, data_path);
    set_history(&mut santas, data_path);
    for santa_name in santas.keys() {
        possible_recipients.insert(
            santa_name.clone(),
            set_possible_recipients(santa_name, &santas, Some(1)),
        );
    }
    return (santas, possible_recipients);
}

pub fn generate(data_path: &PathBuf) {
    let (santas, connections) = make_santas(&data_path);

    for santa in santas {
        println!("{:#?}", santa);
    }
    for (santa_name, recips) in connections.iter() {
        println!("{}: {:#?}", santa_name, recips);
    }
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::env;
    #[test]
    fn test_get_santa_data() {
        let data_path = env::current_dir().unwrap().join("src/test_data");
        // println!("data_path: {:#?}", data_path);

        let (santas, connections) = make_santas(&data_path);
        assert_eq!(santas.len(), 12);

        let groups = vec!["Simpson", "Flanders", "Van Houten"];

        for name in santas.keys() {
            let santa = &santas[name];
            assert!(groups.contains(&&santa.clan.as_str()));
            assert!(santa.email_address.ends_with("@example.com"));
        }

        for (santa_name, recips) in connections.iter() {
            assert!(!recips.contains(santa_name));
        }
    }
}
