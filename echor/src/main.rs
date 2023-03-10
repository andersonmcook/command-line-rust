use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
        .about("Rust echo")
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
                .action(ArgAction::SetTrue),
        )
        .author("Anderson Cook <andersonmcook@gmail.com>")
        .version("0.1.0")
        .get_matches();

    let text: Vec<String> = matches
        .get_many("text")
        .expect("text is required")
        .cloned()
        .collect();

    let ending = if matches.get_flag("omit_newline") {
        ""
    } else {
        "\n"
    };

    print!("{}{}", text.join(" "), ending);
}
