mod arguments;
mod audio;

use std::ffi::OsStr;
use std::process::exit;
use std::time::SystemTime;
use archipelago_rs::{Connection, ConnectionOptions, ConnectionState, Error, Event, ItemHandling};
use structopt::StructOpt;
use sysinfo::System;
use crate::arguments::{Command, Config};
use crate::audio::AudioManager;

const SOUND: &[u8] = include_bytes!("death-bell.wav");

fn main() {
    // const DEATH_SOUND: &[u8] = include_bytes!("death_sound.wav");
    match Command::from_args() {
        Command::Run {
            use_process_name,
            target_process,
            ap_host,
            ap_slot,
            ap_pass,
            disable_sound,
        } => {
            ap_loop(&Config {
                use_process_name,
                target_process,
                ap_host,
                ap_slot,
                ap_pass,
                disable_sound
            })
        }
        Command::ListProcesses => {
            print_processes()
        }
    }
}

fn ap_loop(config: &Config) {
    let mut connection = new_connection(config);
    let audio = AudioManager::new();
    loop {
        match connection.state() {
            ConnectionState::Disconnected(err) => {
                eprintln!("Disconnected: {}", err);
                eprintln!("Please Restart");
                exit(1);
                //connection = new_connection();
            }
            _ => {
                for event in connection.update() {
                    match event {
                        Event::Connected => { println!("Connected Succesfully!")}
                        Event::Error(err) => {
                            match err {
                                Error::ClientDisconnected => {
                                    println!("Disconnected, trying reconnect... (I hope)");

                                }
                                Error::Elsewhere => {
                                    println!("Error: Elsewhere -  {:?}", connection.state().state_type());
                                }
                                _ => {
                                    println!("Error: {:?}", err);
                                }
                            }}
                        Event::DeathLink { cause, source, time, .. } => {
                            if !config.disable_sound {
                                audio.play_sound();
                            }
                            on_death(cause, source, time, config);
                        }
                        _ => { }
                    }
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(5));

    }
}

fn new_connection(config: &Config) -> Connection<()> {
    Connection::new(
        &config.ap_host,
        &config.ap_slot,
        None::<String>,
        ConnectionOptions::new()
            .password(&config.ap_pass)
            .tags(vec!["Tracker", "DeathLink", "CrashLink"])
            .receive_items(ItemHandling::None)
    )
}

fn on_death(cause: Option<String>, source: String, _time: SystemTime, config: &Config) {
    match cause {
        Some(death_cause) => {
            println!("{} Died: {}", source, death_cause);
        }
        None => {
            println!("{} Died", source);
        }
    }
    kill_game(&*config.target_process, config.use_process_name);
}

fn kill_game(game: &str, name_mode: bool) {
    println!("Killing Process {}", game);
    let mut sys = System::new();
    sys.refresh_all();
    for (_, process) in System::processes(&sys) {
        if name_mode {
            if process.name() == OsStr::new(game) {
                process.kill();
            }
        } else if let Some(path) = process.exe() &&  let Some(exe) = path.file_stem() {
            if exe == OsStr::new(game) {
                process.kill();
            }
        }
    }
}

fn print_processes() {
    let mut sys = System::new();
    sys.refresh_all();
    for (_, process) in sys.processes() {
        if let Some(exe) = process.exe() && let Some(name) = exe.file_stem() {
            println!("Name: {} | Exe: {}", process.name().display(), name.display());
        } else {
            print!("Name: {}", process.name().display());
        }
    }
}
