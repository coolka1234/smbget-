extern crate clap;
extern crate indicatif;
use indicatif::{ProgressBar, ProgressStyle};
use clap::{Arg, App};
fn main() {
    let matches = App::new("smbup")
        .version("0.0.1")
        .author("Krzysztof Kulka <krzysztof.kulka1234@gmail.com>")
        .about("samba cli tool for simple archivizng to local samaba server")
        .arg(Arg::with_name("IP")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("IP address of the samba server"))
        .get_matches();
    let url = matches.value_of("IP").unwrap();
    println!("{}", url);
}
fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    let bar = match quiet_mode {
        true => ProgressBar::hidden(),
        false => {
            match length {
                Some(len) => ProgressBar::new(len),
                None => ProgressBar::new_spinner(),
            }
        }
    };

    bar.set_message(msg);
    match length.is_some() {
        true => bar
            .set_style(ProgressStyle::default_bar()
                .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}")
                .progress_chars("=> ")),
        false => bar.set_style(ProgressStyle::default_spinner()),
    };

    bar
}