extern crate clap;

use std::{env, process};

use clap::{App, Arg};

#[derive(Clone, Debug)]
pub struct ConfigArgs {
    pub payload: String,
    pub command: String,
    pub url: String,
    pub header_name: String,
    pub output: String,
    pub format: String,
    pub list: bool,
}

impl ConfigArgs {
    pub fn new() -> Self {
        let mut app = App::new("ysoserial_rs")
            .version("0.0.1")
            // .about("about: Community based web fingerprint analysis tool.")
            .author("author: Kali-Team")
            .arg(
                Arg::new("payload")
                    .short('p')
                    .long("payload")
                    .value_name("PAYLOAD")
                    .help("Select a payload"),
            )
            .arg(
                Arg::new("command")
                    .short('c')
                    .long("command")
                    .value_name("COMMAND")
                    .help("Command to execute"),
            )
            .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("OUTPUT")
                    .help("Save payload to file"),
            )
            .arg(
                Arg::new("url")
                    .long("url")
                    .value_name("URL")
                    .help("URL to request DNS"),
            )
            .arg(
                Arg::new("format")
                    .long("format")
                    .short('f')
                    .value_name("FORMAT")
                    .possible_values(["hex", "base64"])
                    .help("Format to Hex or Base64"),
            )
            .arg(
                Arg::new("header_name")
                    .long("header_name")
                    .value_name("ECHO HEADER NAME")
                    .help("Tomcat echo request header name"),
            )
            .arg(
                Arg::new("list")
                    .long("list")
                    .short('l')
                    .takes_value(false)
                    .help("List all payload"),
            );
        if env::args().len() == 1 {
            app.print_long_help().unwrap_or_default();
            process::exit(0);
        }
        let args = app.get_matches();
        let mut payload: String = String::new();
        let mut command: String = String::new();
        let mut url: String = String::new();
        let mut header_name: String = String::new();
        let mut output: String = String::new();
        let mut format: String = String::new();
        let mut list: bool = false;
        if args.is_present("list") {
            list = true;
        }
        if let Some(p) = args.value_of("payload") {
            payload = p.to_string();
        };
        if let Some(c) = args.value_of("command") {
            command = c.to_string();
        };
        if let Some(u) = args.value_of("url") {
            url = u.to_string();
        };
        if let Some(h_n) = args.value_of("header_name") {
            header_name = h_n.to_string();
        };
        if let Some(o) = args.value_of("output") {
            output = o.to_string();
        };
        if let Some(f) = args.value_of("format") {
            format = f.to_string();
        };
        ConfigArgs {
            payload,
            command,
            url,
            header_name,
            output,
            format,
            list,
        }
    }
}
