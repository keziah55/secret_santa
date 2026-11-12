use chrono::Datelike;
use rand::seq::IteratorRandom;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

mod email;
mod types;

use types::{SantaMap, Santa};


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

/// Return path to `history.csv`
///
/// Panic if file not found.
///
/// /// # Arguments
///
/// * `data_path` - Path to dir containing csv files
///
/// # Returns
///
/// * `PathBuf` - Path to `history.csv`
fn get_history_csv(data_path: &PathBuf) -> PathBuf {
    let csv_file = data_path.join("history.csv");
    if !csv_file.exists() {
        panic!("No 'history.csv' file found: {:?}", csv_file);
    }
    return csv_file;
}

/// Read `history.csv`
///
/// # Arguments
///
/// * `data_path` - Path to dir containing csv files
///
/// # Returns
///
/// * `Vec<String>` - Vector of header items
/// * `Vec<String>` - Vector of lines. Note these are whole lines and are not split by
///   value separator
fn read_history_csv(data_path: &PathBuf) -> (Vec<String>, Vec<String>) {
    let csv_file = get_history_csv(data_path);

    let text = fs::read_to_string(csv_file).unwrap();
    let mut lines_iter = text.split("\n");

    // get header row from `lines`
    let header: Vec<String> = lines_iter
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.to_string())
        .collect();

    let lines: Vec<String> = lines_iter.map(|s| s.to_string()).collect();

    return (header, lines);
}

/// Append `text` to `history.csv`
fn append_history_csv(data_path: &PathBuf, text: String) {
    let csv_file = get_history_csv(data_path);
    let mut f = File::options().append(true).open(csv_file).unwrap();
    writeln!(&mut f, "{text}").unwrap();
}

/// Read `history.csv` and update `santas` HashMap
///
/// * `santas` - HashMap of all Santas; `history` fields are populated by this function
/// * `data_path` - Path to dir containing csv files
fn set_history(santas: &mut SantaMap, data_path: &PathBuf, history_size: usize) {
    let (header, lines) = read_history_csv(data_path);

    // `start` and `end` are row numbers to get history from
    let end = lines.len() - 1;
    let start = if history_size > end {
        0
    } else {
        end - history_size
    };

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

        for row_idx in start..end {
            if lines[row_idx].trim().is_empty() {
                continue;
            }

            let row: Vec<&str> = lines[row_idx].split(",").collect();
            if row[idx] != "" {
                santa.history.push(row[idx].to_string().clone());
            }
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

    append_history_csv(data_path, new_line_values.join(","));
}

/// Fill `recips` vec with possible recipients for last name in `gift_chain`
///
/// # Arguments
///
/// * `recips` - Vector to write recipients to
/// * `santas` - HashMap of all Santas
/// * `gift_chain` - Current gift chain. The last item is taken as the Santa to get
///    recipients for
fn get_possible_recipients(recips: &mut Vec<String>, santas: &SantaMap, gift_chain: &Vec<String>) {
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

        // check if `santa` has `person` in recent history
        if santa.history.contains(&person.name) {
            continue;
        }
        recips.push(key.clone());
    }
}

/// Return true if `recipient` is valid for `santa`
fn is_valid_recipient(santa_name: &String, recipient_name: &String, all_santas: &SantaMap) -> bool {
    let santa: &Santa = &all_santas[santa_name];
    let recipient: &Santa = &all_santas[recipient_name];
    return (santa.clan != recipient.clan) && !santa.history.contains(&recipient.name);
}

/// Make HashMap of all Santa info, from csv files.
///
/// # Arguments
///
/// * `data_path` - Path to dir containing `family.csv` and `history.csv`
fn make_santas(data_path: &PathBuf, history_size: usize) -> SantaMap {
    let mut santas: SantaMap = HashMap::new();

    read_family_csv(&mut santas, data_path);
    set_history(&mut santas, data_path, history_size);

    return santas;
}

/// Make vector of names, defining secret santa gift chain
///
/// # Arguments
///
/// * `santas` - All santa info
///
/// # Returns
///
/// * `Vec<String>` - Vector defining gift chain
fn make_gift_chain(santas: &SantaMap) -> Vec<String> {
    let mut known_invalid_chain: HashSet<String> = HashSet::new();

    let mut gift_chain_opt: Option<Vec<String>> = None;
    while gift_chain_opt.is_none() {
        gift_chain_opt = try_make_gift_chain(santas, &mut known_invalid_chain);
    }

    println!(
        "Found valid gift chain and {} invalid chains",
        known_invalid_chain.len()
    );
    return gift_chain_opt.unwrap();
}

/// Make a gift chain, either valid or invalid.
///
/// # Arguments
///
/// * `santas` - All santa info
/// * `known_invalid_chain` - Vector of known invalid chains, as strings.
///   If gift chain found by this function is not valid, it's string representation is
///   added to the set before returning.
///
/// # Returns
///
/// * `Option<Vec<String>>` - Gift chain if valid; None if invalid.
fn try_make_gift_chain(
    santas: &SantaMap,
    known_invalid_chain: &mut HashSet<String>,
) -> Option<Vec<String>> {
    let mut rng = rand::rng();

    let mut gift_chain: Vec<String> = vec![];

    let start = santas.keys().choose(&mut rng).unwrap();
    gift_chain.push(start.to_string());

    while gift_chain.len() < santas.len() {
        let mut recips: Vec<String> = vec![];
        get_possible_recipients(&mut recips, &santas, &gift_chain);

        let gift_chain_str = gift_chain.join("");

        if recips.is_empty() || known_invalid_chain.contains(&gift_chain_str) {
            known_invalid_chain.insert(gift_chain_str);
            return None;
        } else {
            let recip = recips.into_iter().choose(&mut rng).unwrap();
            gift_chain.push(recip);
        }
    }

    // check that last in chain can give to first
    if !is_valid_recipient(
        &gift_chain.last().unwrap(),
        &gift_chain.first().unwrap(),
        santas,
    ) {
        let gift_chain_str = gift_chain.join("");
        known_invalid_chain.insert(gift_chain_str);
        return None;
    }
    return Some(gift_chain);
}

pub fn generate(data_path: &PathBuf, history_size: usize, do_update_history: bool) {
    let santas = make_santas(&data_path, history_size);
    let gift_chain = make_gift_chain(&santas);

    for (n, name) in gift_chain.iter().enumerate() {
        let santa: &Santa = &santas[name];
        let recip_idx = if n == gift_chain.len() - 1 { 0 } else { n + 1 };
        let recip: &Santa = &santas[&gift_chain[recip_idx]];

        if !is_valid_recipient(&santa.name, &recip.name, &santas) {
            panic!(
                "Invalid gift chain! Santa {} cannot give to {}",
                &santa.name, &recip.name
            );
        }

        // email::send_email(&santas["Keziah"], &santas["Keziah"], data_path);

        // let img = email::make_email(&santa, &recip);
        // println!("\n{}: {}", santa.name, santa.email_address);
        // println!("{}", img);
    }

    email::send_email(&santas["Keziah"], &santas["Keziah"], data_path);

    if do_update_history {
        let year = format!("{}", chrono::Utc::now().year());
        update_history(data_path, &gift_chain, year);
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

        let santas = make_santas(&data_path, 0);
        assert_eq!(santas.len(), 12);

        let groups = vec!["Simpson", "Flanders", "Van Houten"];

        for name in santas.keys() {
            let santa = &santas[name];
            assert!(groups.contains(&&santa.clan.as_str()));
            assert!(santa.email_address.ends_with("@example.com"));
        }
    }
}
