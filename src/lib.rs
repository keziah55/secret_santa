use polars::prelude::*;
use polars_core::prelude::*;
use polars_io::prelude::*;
use std::collections::HashMap;
use std::path::{Path,PathBuf};

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
fn read_csv(filename: PathBuf) -> Result<DataFrame, PolarsError> {
    CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(Some(filename))?
            .finish()
}

/// Read all csv files from `data_path`
///
/// # Arguments
///
/// * `data_path` - Dir containing `family.csv` and `history.csv`
fn read_data_from_csv(data_path: &Path) -> (DataFrame, DataFrame) {
    let family_csv = data_path.join("family.csv");
    assert!(family_csv.exists());

    let history_csv = data_path.join("history.csv");
    assert!(history_csv.exists());

    let family_df = read_csv(family_csv).unwrap();
    let history_df = read_csv(history_csv).unwrap();

    // let names_from_family = family_df.into_columns().iter().map(|col| col.name()).collect();

    return (family_df, history_df);
}

/// Make `Santa` structs for all family members.
///
/// # Arguments
///
/// * `data_path` - Dir containing `family.csv` and `history.csv`
fn get_santa_data(data_path: &Path) -> HashMap<String, Santa> {
    let mut santas: HashMap<String, Santa> = HashMap::new();

    let (family_df, history_df) = read_data_from_csv(data_path);

    for idx in 0..family_df.height() {
        let row = family_df.get(idx).unwrap();

        // let history = history_df
        //     .column(row["name"])
        //     .unwrap()
        //     .str()
        //     .unwrap()
        //     .into_iter()
        //     .collect();

        let santa = Santa {
            name: row["name"],
            clan: row["group"],
            email_address: row["email_address"],
            history: None,
        };
        santas.insert(santa.name, santa);
    }

    return santas;

    // for column in history_df.columns() {
    //     if column.name() == "" {
    //         continue;
    //     }

    //     let history: Vec<Option<Santa>> = column.str()?.into_iter().collect();

    //     for idx in column.len() {
    //         match column.get(idx).unwrap() {}
    //     }

    //     let mut history: Vec<Option<Santa>>;
    //     history_df.column(row["name"]).unwrap();
    // }
}

pub fn generate(data_path: &Path) {
    let santas = get_santa_data(data_path);
    for santa in santas {
        println!("{:#?}", santa);
    }
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[test]
    fn test_get_santa_data() {
        let data_path = Path();
    }
}
