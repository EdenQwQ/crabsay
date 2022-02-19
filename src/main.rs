use clap::{arg, App};

fn main() {
    let args = args().get_matches();

    let text: String = args.value_of("text").unwrap().to_string();
    let think: bool = args.is_present("think");
    let eyes: String = args.value_of("eyes").unwrap().to_string();
    let mouth: String = args.value_of("mouth").unwrap().to_string();
    let seperator: char = args.value_of("seperator").unwrap().chars().next().unwrap();

    let mut output: Vec<String> = Vec::new();

    let lines = text.split(seperator).collect::<Vec<&str>>();
    let mut length = 0;
    for line in &lines {
        if line.len() > length {
            length = line.len();
        }
    }

    output.push("_".repeat(length + 2));
    output.push("".to_string());
    for line in lines {
        // output each line in the center of < > and with the correct length
        output.push(format!(" {: ^1$} ", line, length));
    }
    output.push("_".repeat(length + 2));

    if think {
        output.push(
            "        o
         o
          o"
            .to_string(),
        );
    } else {
        output.push(
            "        \\
         \\
          \\"
            .to_string(),
        );
    }

    output.push(format!(
        "            _~^~^~_
        \\) /  {}  \\ (/
          '_   {}   _'
          / '-----' \\
",
        eyes, mouth
    ));

    for line in output {
        println!("{}", line);
    }
}

fn args() -> App<'static> {
    App::new("crabsay")
        .version("0.1.0")
        .author("EdenQwQ <lsahlm1eden@gmail.com>")
        .about("Cowsay rewriten in Rust but with Ferris")
        .arg(arg!([text] "Text to say/think"))
        .arg(
            arg!(-t - -think)
                .required(false)
                .takes_value(false)
                .help("If set, ferris will think rather than say."),
        )
        .arg(
            arg!(-e - -eyes)
                .required(false)
                .takes_value(true)
                .help("Sets the eyes of the ferris")
                .default_value("o o"),
        )
        .arg(
            arg!(-m - -mouth)
                .required(false)
                .takes_value(true)
                .help("Sets the mouth of the ferris")
                .default_value("-"),
        )
        .arg(
            arg!(-s - -seperator)
                .required(false)
                .takes_value(true)
                .help("Sets the seperator of text lines.")
                .default_value("\n"),
        )
}
