extern crate clap;
extern crate indicatif;
use indicatif::{ProgressBar, ProgressStyle};
use clap::{Arg, App};
fn main() {
     let matches = App::new("samba_cli")
        .version("1.0")
        .author("Krzysztof Kulka <krzysztof.kulka1234@gmail.com>")
        .about("Copies files to a Samba server")
        .arg(Arg::with_name("source")
             .help("Sets the source file to use")
             .required(true)
             .index(1))
        .arg(Arg::with_name("destination")
             .help("Sets the destination path on the Samba server")
             .required(true)
             .index(2))
        .arg(Arg::with_name("username")
             .help("Samba server username")
             .required(true)
             .short("u")
             .long("username"))
        .arg(Arg::with_name("password")
             .help("Samba server password")
             .required(true)
             .short("p")
             .long("password"))
        .arg(Arg::with_name("server")
             .help("Samba server address")
             .required(true)
             .short("s")
             .long("server"))
        .arg(Arg::with_name("share")
             .help("Share name on the Samba server")
             .short("s")
             .long("share"))
        .get_matches();
    let source = matches.value_of("source").unwrap();
    let destination = matches.value_of("destination").unwrap();
    let username = matches.value_of("username").unwrap();
    let password = matches.value_of("password").unwrap();
    let server = matches.value_of("server").unwrap();
    let share = matches.value_of("share").unwrap_or("share");

    // Call function to copy file to Samba server
    copy_to_samba(source, destination, username, password, server, share);   
     
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
extern crate pavao;
use pavao::{SmbClient, SmbCredentials, SmbOptions};
use std::fs::File;
use std::io::Read;

fn copy_to_samba(source: &str, destination: &str, username: &str, password: &str, server: &str, share: &str) {
    // Read the source file
    let mut file = File::open(source).expect("Failed to open source file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read source file");

    // Create SMB client
    let mut client = SmbClient::new(
    SmbCredentials::default()
        .server(server)
        .username(username)
        .share(share)
        .password(password)
    ,SmbOptions::default().one_share_per_server(true),
    )
    .unwrap();
        // .username(username)
        // .password(password)
        // .connect()
        // .expect("Failed to connect to Samba server");

    // Write the file to the destination on the Samba server
    client
        .write(destination, &buffer)
        .expect("Failed to write file to Samba server");
}
