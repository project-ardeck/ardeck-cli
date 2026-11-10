use std::io::Write;

use enigo::Keyboard;

use crate::config::load_config;

mod config;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .target(env_logger::Target::Stdout)
        .init();

    let profiles = match load_config() {
        Some(config) => config,
        None => {
            eprintln!("Failed to load config file.");
            panic!()
        }
    };

    let ports = ardeck::available_list();

    let port_names: Vec<String> = ports
        .iter()
        .map(|item| item.port_name().to_string())
        .collect();

    if port_names.is_empty() {
        eprintln!("COMポートが一つも接続されていません。");
        std::process::exit(0)
    }

    let selected_port_name = match inquire::Select::new("ポートを選択", port_names).prompt() {
        Ok(port) => port,
        Err(_e) => {
            println!("キャンセルされたか、エラーが発生しました。");
            std::process::exit(0)
        }
    };

    let profile_names: Vec<String> = profiles
        .iter()
        .map(|item| item.preset_name().to_string())
        .collect();

    let selected_profile_name =
        match inquire::Select::new("プロファイルを選択", profile_names).prompt() {
            Ok(profile) => profile,
            Err(_e) => {
                println!("キャンセルされたか、エラーが発生しました。");
                std::process::exit(0)
            }
        };

    let selected_profile = match profiles
        .iter()
        .find(|profile| profile.preset_name() == selected_profile_name)
    {
        Some(profile) => profile,
        None => {
            eprintln!("何らかの理由でプロファイルを設定できませんでした。");
            panic!()
        }
    };

    let ardeck = ardeck::Connection::new(selected_port_name, 9600).unwrap();

    let rx = ardeck.action_receiver();
    let rx = rx.lock().unwrap();
    let exit_flag = ardeck.exit_flag();

    ardeck
        .conrol_sender()
        .send(ardeck::ControlMessage::RequestStartTriggeredContinuousMode)
        .unwrap();

    let mut enigo = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    loop {
        if exit_flag.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }
        if let Ok(switch_info) = rx.recv() {
            println!("{:?}", switch_info);
            for switch in switch_info {
                println!("{:?}", switch);
                let target: Vec<&config::Mapping> = selected_profile
                    .mappinglist()
                    .iter()
                    .filter(|mapping| {
                        mapping.switch_kind() == switch.kind()
                            && mapping.switch_id() == switch.pin()
                    })
                    .collect();

                for map in target {
                    if switch.state() == 1 {
                        let key = map.press_key();
                        if let Err(_e) =
                            enigo.key(enigo::Key::Unicode(key), enigo::Direction::Click)
                        {
                            eprintln!("Failed to press key.");
                        };
                    }
                }
            }
        }
    }
}
