use rand::seq::IteratorRandom;
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

type SantaMap = HashMap<String, Santa>;

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

/// Fill `recips` vec with possible recipients for `santa_name`.
/// 
/// # Arguments
/// 
/// `recips` - Vector to write recipients to
/// `santa_name` - Name of Santa for these recipients
/// `santas` - HashMap of all Santas
/// `gift_chain` - Current gift chain
/// `exclude_size` - Number of history years to exclude
fn get_possible_recipients(
    recips: &mut Vec<String>,
    santa_name: &String,
    santas: &SantaMap,
    gift_chain: &Vec<String>,
    exclude_size: usize,
) {
    let santa = &santas[santa_name];

    for key in santas.keys() {
        if *key == santa.name {
            continue;
        }

        let person = &santas[key];

        if person.clan == santa.clan || gift_chain.contains(&person.name) {
            // person is in same group as santa or is already allocated
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

    // return recips;
}

fn make_santas(data_path: &PathBuf) -> SantaMap {
    let mut santas: SantaMap = HashMap::new();

    read_family_csv(&mut santas, data_path);
    set_history(&mut santas, data_path);

    return santas;
}

pub fn generate(data_path: &PathBuf) {
    let mut rng = rand::rng();

    let santas = make_santas(&data_path);

    let mut gift_chain: Vec<String> = vec![];

    let start = santas.keys().choose(&mut rng).unwrap();
    gift_chain.push(start.to_string());

    let mut recips: Vec<String> = vec![];

    get_possible_recipients(&mut recips, &start, &santas, &gift_chain, 1);

    if (recips.len() == 0) {
        println!("No valid recipients for {}. Backtracking.", *start);
    } else {
        let recip = recips.into_iter().choose(&mut rng).unwrap();
        println!("{} -> {}", *start, recip);
    }

    // for santa in santas {
    //     println!("{:#?}", santa);
    // }

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

        let santas = make_santas(&data_path);
        assert_eq!(santas.len(), 12);

        let groups = vec!["Simpson", "Flanders", "Van Houten"];

        for name in santas.keys() {
            let santa = &santas[name];
            assert!(groups.contains(&&santa.clan.as_str()));
            assert!(santa.email_address.ends_with("@example.com"));
        }
    }
}
