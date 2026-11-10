use chrono::Datelike;
use rand::seq::IteratorRandom;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone)]
struct Santa {
    name: String,
    clan: String,
    email_address: String,
    history: Vec<String>,
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
/// * `exclude_size` - Number of history years to exclude
fn get_possible_recipients(
    recips: &mut Vec<String>,
    santas: &SantaMap,
    gift_chain: &Vec<String>,
    extra_exclude: &Vec<String>,
) {
    let santa_name = gift_chain.last().unwrap();
    let santa = &santas[santa_name];

    for key in santas.keys() {
        if *key == santa.name {
            continue;
        }

        let person = &santas[key];

        if person.clan == santa.clan
            || gift_chain.contains(&person.name)
            || extra_exclude.contains(&person.name)
        {
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

fn check_possible_santas_valid(possible_santas: &HashSet<String>, santas: &SantaMap) -> bool {
    match possible_santas.len() {
        0 => return false,
        1 => return true,
        _ => {
            let mut groups: HashSet<&String> = HashSet::new();

            for name in possible_santas {
                groups.insert(&santas[name].clan);
            }

            return groups.len() > 1;
        }
    }
}

/// Make vector of names, defining secret santa gift chain
fn make_gift_chain(santas: &SantaMap) -> Vec<String> {
    let mut gift_chain: Vec<String> = vec![];

    let mut rng = rand::rng();

    let mut all_recips: HashSet<String> = santas.keys().cloned().collect();

    let start = santas.keys().choose(&mut rng).unwrap();
    gift_chain.push(start.to_string());
    all_recips.remove(start);

    let mut extra_exclude = vec![];

    // TODO check final item in chain can gift to first
    let mut i = 0;
    while gift_chain.len() < santas.len() {
        if i > 25 {
            println!("max iterations reached");
            break;
        } else {
            i += 1;
        }

        println!();

        let mut recips: Vec<String> = vec![];
        get_possible_recipients(&mut recips, &santas, &gift_chain, &extra_exclude);

        if recips.is_empty() || !check_possible_santas_valid(&all_recips, &santas) {
            // TODO handle invalid chain here
            let removed = gift_chain.pop().unwrap();
            println!(
                "No valid recipients for {}. Backtracking. recips={:#?}, all_recips={:#?}",
                removed, recips, all_recips
            );

            let key = removed.clone();
            extra_exclude.push(removed);
            all_recips.insert(key);
        } else {
            extra_exclude.clear();
            let recip = recips.into_iter().choose(&mut rng).unwrap();
            all_recips.remove(&recip);
            gift_chain.push(recip);

            println!(
                "Added {} to gift chain. Remaining recipients in draw:",
                &gift_chain.last().unwrap()
            );
            for name in &all_recips {
                println!("  {}", name);
            }
        }
    }
    return gift_chain;
}

pub fn generate(data_path: &PathBuf, history_size: usize, do_update_history: bool) {
    let santas = make_santas(&data_path, history_size);
    let gift_chain = make_gift_chain(&santas);

    println!("Gift chain:");
    for (n, name) in gift_chain.iter().enumerate() {
        print!("{}", name);
        if n < gift_chain.len() - 1 {
            print!(" -> ");
        }
    }
    println!();

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
