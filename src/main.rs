extern crate clap;
extern crate dirs;

use clap::{App, Arg};
use kx::Config;
use skim::{Skim, SkimOptionsBuilder};
use std::fs;
use std::io::Cursor;
use std::process;

fn main() {
    let matches = App::new("kx")
        .version("1.0")
        .author("onatm - https://github.com/onatm")
        .about("Interactively switch between kubernetes contexts without any external dependencies")
        .arg(Arg::with_name("NAME").help("Switch to context <NAME>"))
        .arg(
            Arg::with_name("current")
                .short("c")
                .long("current")
                .help("Show the current context"),
        )
        .get_matches();

    let home_dir = dirs::home_dir().unwrap_or_else(|| {
        println!("Cannot find HOME directory");
        process::exit(1);
    });

    let kube_config_folder_path = &home_dir.join(".kube");

    let kube_config_path = &kube_config_folder_path.join("config");

    let contents = &fs::read_to_string(kube_config_path).unwrap_or_else(|err| {
        println!("Cannot read kube config: {}", err);
        process::exit(1);
    });

    let contents = contents.lines().collect::<Vec<&str>>();

    let kube_config = &mut Config::load(contents).unwrap_or_else(|err| {
        println!("Cannot read kube config: {}", err);
        process::exit(1);
    });

    if let Some(new_context) = matches.value_of("NAME") {
        println!("new context {}", new_context);
    }

    if matches.is_present("current") {
        let current_context = kube_config.get_current_context().unwrap_or_else(|err| {
            println!("error: {}", err);
            process::exit(1);
        });

        println!("{}", current_context);
    }

    let contexts = kube_config.list_contexts();

    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(true)
        .build()
        .unwrap();

    let selected_items = Skim::run_with(&options, Some(Box::new(Cursor::new(contexts))))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    let selected_context = selected_items
        .iter()
        .next()
        .unwrap_or_else(|| {
            println!("Context not selected");
            process::exit(1);
        })
        .get_output_text();

    let new_context = format!("current-context: {}", &selected_context);

    kube_config
        .set_current_context(&new_context)
        .unwrap_or_else(|err| {
            println!("Cannot set current-context: {}", err);
            process::exit(1);
        });

    fs::write(kube_config_path, kube_config.get_config()).unwrap_or_else(|err| {
        println!("Cannot save kubeconfig: {}", err);
        process::exit(1);
    });

    println!("Switched to context \"{}\"", selected_context);
}
