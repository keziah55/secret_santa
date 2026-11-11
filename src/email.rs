/// Make ASCII image with `name` subbed in
pub fn make_img(name: &String) -> String {
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

    let mut image_str: Vec<String> = vec![];
    for line in image {
        if line.is_empty() {
            image_str.push(c.repeat(width));
        } else {
            let s = format!("{} {}", c, line);
            let pad = width - s.len() - 1;
            image_str.push(format!("{}{}{}", s, " ".repeat(pad), c));
        }
    }
    return image_str.join("\n");
}