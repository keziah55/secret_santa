use rand::seq::IteratorRandom;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
struct Santa {
    name: String,
    clan: String,
    email_address: String,
    history: Vec<Option<String>>,
}

type SantaMap = HashMap<String, Santa>;

/// Read csv file and update `santas` HashMap
///
/// # Arguments
///
/// * `santas` - Mutable reference to (empty) HashMap
/// * `data_path` - Path to dir containing csv files
fn read_family_csv(santas: &mut SantaMap, data_path: &PathBuf) {
    let csv_file = data_path.join("family.csv");
    if !csv_file.exists() {
        panic!("No 'family.csv' file found: {:?}", csv_file);
    }

    let text = fs::read_to_string(csv_file).unwrap();
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

fn read_history_csv(data_path: &PathBuf) -> (Vec<String>, Vec<String>) {
    let csv_file = data_path.join("history.csv");
    if !csv_file.exists() {
        panic!("No 'history.csv' file found: {:?}", csv_file);
    }

    let text = fs::read_to_string(csv_file).unwrap();
    let lines: Vec<String> = text.split("\n").map(|s| s.to_string()).collect();

    // get header row from `lines`
    let header: Vec<String> = lines
        .first()
        .unwrap()
        .split(",")
        .map(|s| s.to_string())
        .collect();

    return (header, lines);
}

/// Read `history.csv` and update `santas` HashMap
///
/// * `santas` - HashMap of all Santas; `history` fields are populated by this function
/// * `data_path` - Path to dir containing csv files
fn set_history(santas: &mut SantaMap, data_path: &PathBuf) {
    let (header, lines) = read_history_csv(data_path);

    // iterate over columns of data
    for idx in 0..header.len() {
        let name = &header[idx];
        if name.trim().is_empty() {
            continue;
        }

        if !santas.contains_key(name) {
            panic!("History contains unknown name {}", name);
        }
        let santa = santas.get_mut(name).unwrap();

        for row_idx in (1..lines.len()).rev() {
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

/// Update `history.csv` with new data
fn update_history(data_path: &PathBuf, gift_chain: &Vec<String>, year: String) {
    let (header, _) = read_history_csv(data_path);

    let mut new_line_values = vec![year];

    for name in header {
        if name.trim().is_empty() {
            continue;
        }

        // find n+1 from gift_chain
        let mut idx = gift_chain.iter().position(|s| s == &name).unwrap();
        if idx == gift_chain.len() - 1 {
            idx = 0;
        } else {
            idx += 1;
        }
        new_line_values.push(gift_chain[idx].clone());
    }

    let new_line = new_line_values.join(",");

    let csv_file = data_path.join("history.csv");
    let mut f = File::options().append(true).open(csv_file).unwrap();
    writeln!(&mut f, "{new_line}").unwrap();
}

/// Fill `recips` vec with possible recipients for `santa_name`.
///
/// # Arguments
///
/// * `recips` - Vector to write recipients to
/// * `santas` - HashMap of all Santas
/// * `gift_chain` - Current gift chain. The last item is taken as the Santa to get
///    recipients for
/// * `exclude_size` - Number of history years to exclude
fn get_possible_recipients(
    recips: &mut Vec<String>,
    santas: &SantaMap,
    gift_chain: &Vec<String>,
    exclude_size: usize,
) {
    let santa_name = gift_chain.last().unwrap();
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
            for (idx, item) in santa.history.iter().enumerate() {
                if idx < exclude_size && item.is_some() {
                    if item.as_deref().unwrap() == person.name {
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

/// Make HashMap of all Santa info, from csv files.
///
/// # Arguments
///
/// * `data_path` - Path to dir containing `family.csv` and `history.csv`
fn make_santas(data_path: &PathBuf) -> SantaMap {
    let mut santas: SantaMap = HashMap::new();

    read_family_csv(&mut santas, data_path);
    set_history(&mut santas, data_path);

    return santas;
}

pub fn generate(data_path: &PathBuf, do_update_history: bool) {
    let mut rng = rand::rng();

    let santas = make_santas(&data_path);

    let mut gift_chain: Vec<String> = vec![];

    let start = santas.keys().choose(&mut rng).unwrap();
    gift_chain.push(start.to_string());

    // TODO check final item in chain can gift to first
    while gift_chain.len() < santas.len() {
        let mut recips: Vec<String> = vec![];
        get_possible_recipients(&mut recips, &santas, &gift_chain, 1);

        if recips.is_empty() {
            // TODO handle invalid chain here
            let removed = gift_chain.pop().unwrap();
            println!("No valid recipients for {}. Backtracking.", removed);
        } else {
            let recip = recips.into_iter().choose(&mut rng).unwrap();
            gift_chain.push(recip);
        }
    }
    println!("Gift chain:");
    for (n, name) in gift_chain.iter().enumerate() {
        print!("{}", name);
        if n < gift_chain.len() - 1 {
            print!(" -> ");
        }
    }
    println!();
    if do_update_history {
        update_history(data_path, &gift_chain, String::from("2026"));
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
