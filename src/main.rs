extern crate clap;

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
