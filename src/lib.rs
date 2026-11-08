use polars::prelude::*;
use std::collections::HashMap;

struct Santa {
    name: String,
    clan: String,
    email_address: String,
    history: Option<Vec<Option<Santa>>>,
}

fn read_csv(filename: &Path) -> Result<DataFrame, PolarsError> {
    CsvReader::from_path(filename)?.has_header(true).finish()
}

fn read_data_from_csv(data_path: &Path) -> (DataFrame, DataFrame) {
    let family_csv = data_path.join("family.csv");
    assert!(family_csv.exists());

    let history_csv = data_path.join("history.csv");
    assert!(history_csv.exists());

    family_df = read_csv(family_csv);
    history_df = read_csv(history_csv);

    // let names_from_family = family_df.into_columns().iter().map(|col| col.name()).collect();

    return (family_df, history_df);
}

fn get_santa_data(data_path: &Path) -> HashMap<Santa> {
    let mut santas: HashMap<Santa> = HashMap::new();

    let (family_df, history_df) = read_data_from_csv(data_path);

    for idx in 0..family_df.height() {
        let row = df.get(idx).unwrap();

        let history = history_df
            .column(row["name"])
            .unwrap()
            .str()?
            .into_iter()
            .collect();

        let santa = Santa {
            name: row["name"],
            clan: row["group"],
            email_address: row["email_address"],
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
