use std::fs::File;
use std::io::Write;
use std::{io, process};

use ysoserial::cli::ConfigArgs;
use ysoserial::yso;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigArgs::new();
    if config.list {
        yso::list();
    }
    if let Some(p) = &config.payload {
        let payload = yso::get_payload(p);
        match config.format.as_str() {
            "hex" => {
                let s = hex::encode(payload);
                output_format(&config.output, &s);
            }
            "base64" => {
                let s = base64::encode(payload);
                output_format(&config.output, &s);
            }
            _ => output_bytes(&config.output, payload),
        }
    } else {
        println!("请指定Payload");
        process::exit(0);
    }

    Ok(())
}

fn output_format(config_output: &Option<String>, payload: &str) {
    if let Some(output) = &config_output {
        if let Ok(mut file) = File::create(output) {
            file.write_all(payload.as_bytes()).unwrap_or_default();
            println!("写入文件:{},payload大小:{}", output, payload.len());
        } else {
            println!("写入文件失败")
        };
    } else {
        io::stdout()
            .write_all(payload.as_bytes())
            .unwrap_or_default();
    }
}

fn output_bytes(config_output: &Option<String>, payload: Vec<u8>) {
    if let Some(output) = &config_output {
        if let Ok(mut file) = File::create(output) {
            file.write_all(&payload).unwrap_or_default();
            println!("写入文件:{},payload大小:{}", output, payload.len());
        } else {
            println!("写入文件失败")
        };
    } else {
        io::stdout().write_all(&payload).unwrap_or_default();
    }
}
