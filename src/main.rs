extern crate dirs;

use kx::KubeConfig;
use skim::{Skim, SkimOptionsBuilder};
use std::fs;
use std::io::Cursor;
use std::process;

fn main() {
    let home_dir = dirs::home_dir().unwrap_or_else(|| {
        println!("Cannot find HOME directory");
        process::exit(1);
    });

    let kube_config_folder_path = &home_dir.join(".kube");

    let kube_config_path = &kube_config_folder_path.join("config");

    let kube_config_raw = &fs::read_to_string(kube_config_path).unwrap_or_else(|err| {
        println!("Cannot find kube config: {}", err);
        process::exit(1);
    });

    let kube_config: &KubeConfig = &serde_yaml::from_str(kube_config_raw).expect("failed");

    // let kube_config = &KubeConfig::load(kube_config_path).unwrap_or_else(|err| {
    //     println!("Cannot read kube config: {}", err);
    //     process::exit(1);
    // });

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

    // let selected_context = &kube_config
    //     .contexts
    //     .get(selected_context_item.get_index())
    //     .unwrap_or_else(|| {
    //         println!("Cannot get selected context");
    //         process::exit(1);
    //     });

    let selected_context_entry = format!("current-context: {}", selected_context);

    // println!(
    //     "current context: {}",
    //     &kube_config.current_context.as_ref().unwrap()
    // );

    // let tmp_kube_config_path = &kube_config_folder_path.join("config.tmp");

    // fs::copy(kube_config_path, tmp_kube_config_path).unwrap_or_else(|err| {
    //     println!("Cannot copy to temporary file: {}", err);
    //     process::exit(1);
    // });

    if let Some(current_context) = kube_config.current_context.clone() {
        let current_context = format!("current-context: {}", current_context);

        let new_kube_config_raw =
            kube_config_raw.replace(&current_context[..], &selected_context_entry[..]);

        fs::write(kube_config_path, new_kube_config_raw).unwrap_or_else(|err| {
            println!("Cannot set the current context: {}", err);
            process::exit(1);
        });

    // fs::remove_file(tmp_kube_config_path).unwrap_or_else(|err| {
    //     println!("Cannot remove the temporary file: {}", err);
    //     process::exit(1);
    // });
    } else {
        let mut new_kube_config_raw = kube_config_raw.to_owned();
        new_kube_config_raw.push_str(&selected_context_entry[..]);

        fs::write(kube_config_path, new_kube_config_raw).unwrap_or_else(|err| {
            println!("Cannot set the current context: {}", err);
            process::exit(1);
        });
    }
    println!("Switched to context \"{}\"", selected_context);
}
