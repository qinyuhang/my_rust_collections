use clap::App;
use std::env;
/// build all wasm
/// put all web in dist/{crate_name}
use std::fs::{create_dir, read_to_string, remove_dir, remove_dir_all};
use std::process::{Command, Stdio};
//use std::path::Path;
use toml::Value;

//#[macro_use]
//extern crate clap;

fn main() {
    // cwd is not stable when run in defferent place how to get a fix path of the toml file?
    let cargo_config_path = env::current_dir().unwrap().join("../Cargo.toml");
    let dist_path = env::current_dir().unwrap().join("../dist");
    let str = read_to_string(cargo_config_path).unwrap();
    let t = str.parse::<Value>().unwrap();

    // create dist/ dir for subtree
    create_dir(&dist_path).unwrap_or_else(|_| {
        remove_dir_all(&dist_path).unwrap();
        create_dir(&dist_path).unwrap();
    });

    t["workspace"]["members"]
        .as_array()
        .unwrap()
        .iter()
        .for_each(|member_name| {
            // use thread to compile
            println!("{}", member_name);
            // TODO: if is clib or rlib then compile wasm
            // TODO: not span
            Command::new("sh")
                .arg("-c")
                .arg(format!(
                    r#"
pwd;
cd ../{};
wasm-pack build --release --target web;
cp -RL web ../dist/{};
                    "#,
                    member_name.as_str().unwrap(),
                    member_name.as_str().unwrap(),
                ))
                .stdout(Stdio::inherit())
                .output()
                .expect("msg");
        });

}
