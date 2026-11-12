use std::fs;
use std::path::PathBuf;

use lettre::message::{Mailbox, header::ContentType};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use toml::Table;

use crate::types::Santa;

static HTML_HEADER: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>secret Santa!</title>
</head>
"#;

/// Make ASCII image with `name` subbed in
fn make_img(name: &String) -> String {
    let name_str = format!(" \\   \\  {}!", name.to_uppercase());
    let image: Vec<&str> = vec![
        "",
        " .-.",
        "(o o) I am the ghost of Christmas presents,",
        "| O \\  and your secret santa is...",
        &name_str,
        "  `~~~'",
        "",
    ];

    let width = image.iter().map(|s| s.len()).max().unwrap() + 4;
    let c = "#";

    let mut image_str: Vec<String> = vec![String::from(HTML_HEADER), "<body>\n<pre>".to_string()];
    for line in image {
        if line.is_empty() {
            image_str.push(c.repeat(width));
        } else {
            let s = format!("{} {}", c, line);
            let pad = width - s.len() - 1;
            image_str.push(format!("{}{}{}", s, " ".repeat(pad), c));
        }
    }
    image_str.push("</pre>\n</body>\n</html>".to_string());
    return image_str.join("\n");
}

fn get_email_config(data_path: &PathBuf) -> toml::Table {
    let config_file = data_path.join("config.toml");
    if !config_file.exists() {
        panic!("No 'config.toml' file found: {:?}", config_file);
    }
    let config = fs::read_to_string(config_file)
        .unwrap()
        .parse::<Table>()
        .unwrap();

    return config["email"].as_table().unwrap().clone();
}

pub fn send_email(santa: &Santa, recip: &Santa, data_path: &PathBuf) {

    let html = make_img(&recip.name);

    let config = get_email_config(data_path);

    let email: Message = Message::builder()
        .from(Mailbox::new(
            Some(config["name"].to_string().to_owned()),
            config["address"].as_str().unwrap().parse().unwrap(),
        ))
        .to(Mailbox::new(
            Some(recip.name.clone().to_owned()),
            recip.email_address.parse().unwrap(),
        ))
        .subject("Secret Santa!")
        .header(ContentType::TEXT_HTML)
        .body(html)
        .unwrap();

    println!("{}, {}",config["address"].to_string(), config["cred"].to_string().to_owned() );
    let creds = Credentials::new(
        config["address"].to_string().to_owned(),
        config["cred"].to_string().to_owned(),
    );

    // Open a remote connection to gmail
    let mailer = SmtpTransport::starttls_relay(&config["smtp"].as_str().unwrap())
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}
