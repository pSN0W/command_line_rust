use clap::{App,Arg};
fn main() {
    let matches = App::new("rusty_echo")
                            .version("0.1.0")
                            .author("Pratyaksh Singh <09yaksh09@gmail.com>")
                            .about("Implementation of echo in rust")
                            .arg(
                                Arg::with_name("text")
                                    .value_name("TEXT")
                                    .help("Input a text to display")
                                    .required(true)
                                    .min_values(1)
                            )
                            .arg(
                                Arg::with_name("omit_newline")
                                    .short("n")
                                    .help("Do not print new line")
                                    .takes_value(false)
                            )
                            .get_matches();

    let text = match matches.values_of_lossy("text") {
        Some(x) => x,
        None => unreachable!("clap should have thrown error")
    };
    let include_break = matches.is_present("omit_newline");
    let mut final_output = text.join(" ");
    if !include_break {
        final_output += "\n";
    }
    print!("{}",final_output);
}
