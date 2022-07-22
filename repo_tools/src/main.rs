use std::env;
use std::fmt::format;
/// build all wasm
/// put all web in dist/{crate_name}
use std::fs::{create_dir, read_to_string, remove_dir_all};
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

    let html_body = t["workspace"]["members"]
        .as_array()
        .unwrap()
        .iter()
        .map(|member_name| {
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
            format!(
                r#"<li><a href="{}" >{}</a></li>"#,
                member_name.as_str().unwrap(),
                member_name.as_str().unwrap()
            )
        })
        .collect::<String>();
    let html = std::fs::read_to_string("./src/index.html").unwrap_or("".to_string());
    let mut html = html.split("{}");
    let html = html.next().unwrap_or("").to_string()
        + "<ul>"
        + &html_body
        + "</ul>"
        + html.next().unwrap_or("");
    std::fs::write("../dist/index.html", html).unwrap();
}
