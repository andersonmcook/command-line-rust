use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echor")
        .about("Rust echo")
        .version("0.1.0")
        .author("Anderson Cook <andersonmcook@gmail.com>")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .num_args(0),
        )
        .get_matches();

    let text: Vec<String> = matches
        .get_many("text")
        .expect("text is required")
        .map(|s: &String| s.clone())
        .collect();

    let ending = if matches.get_flag("omit_newline") {
        ""
    } else {
        "\n"
    };

    print!("{}{}", text.join(" "), ending);
}
