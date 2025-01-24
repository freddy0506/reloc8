#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use chrono;
use eframe::egui;
use egui::Key;
use log::{debug, info};
use serde;
use serde_json;
use sha256::try_digest;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(serde::Deserialize, PartialEq, Clone, Default, Debug)]
struct TargetPath {
    name: String,
    path: String,
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Sort Download",
        options,
        Box::new(|cc| Ok(Box::new(Content::new(cc)))),
    )
}

fn mov_file(start: String, target: String, filename: String) -> String {
    info!("Trying to move: {} to {}", start, target);

    let start_path = Path::new(&start);
    let target_path = Path::new(&target);

    if !start_path.exists() || !target_path.exists() {
        panic!("One of the given paths doesnt exists");
    }
    debug!("Both paths exist");

    if start_path.parent().unwrap() == target_path {
        info!("Start and target are the same. Nothing todo");
        return start;
    }

    // Adding the filename to the target_path
    let mut m_filename = filename.clone();
    m_filename.push_str(".");
    m_filename.push_str(start_path.extension().unwrap().to_str().unwrap());
    let new_file_path = target_path.join(m_filename);
    debug!("The new filepath will be: {}", new_file_path.display());

    if new_file_path.exists() {
        info!("File already exists");
        if try_digest(new_file_path.clone()).unwrap() == try_digest(start_path).unwrap() {
            info!("They are the same file. Deleting the new one");
            let _ = fs::remove_file(start_path);
        } else {
            info!("There already is a copy. Saving the old one as a hidden file");
            let date = chrono::offset::Local::now();
            let mut new_file_name = ".".to_string();
            new_file_name.push_str(date.format("%Y-%m-%d_%H:%M:%S").to_string().as_str());
            new_file_name.push_str("_");
            new_file_name.push_str(new_file_path.file_name().unwrap().to_str().unwrap());
            let _ = fs::rename(
                new_file_path.clone(),
                new_file_path.with_file_name(new_file_name),
            );
            let _ = fs::rename(start_path, new_file_path.clone());
        }
        return new_file_path.to_str().unwrap().to_string();
    }
    info!("File is being moved to new location");
    let _ = fs::rename(start_path, new_file_path.clone());
    return new_file_path.to_str().unwrap().to_string();
}

struct Content {
    paths: Vec<TargetPath>,
    current_path: TargetPath,
    start_path: String,
    filename: String,
}
impl Content {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let path_data = fs::read_to_string("~/.config/reloc8.json").unwrap();
        let start_path = env::args()
            .collect::<Vec<String>>()
            .get(1)
            .expect("No Path was given")
            .to_string();
        let filename = Path::new(&start_path)
            .file_name()
            .expect("Can't get filename")
            .to_str()
            .unwrap()
            .split('.')
            .next()
            .unwrap()
            .to_string();

        let paths: Vec<TargetPath> = serde_json::from_str(path_data.as_str()).expect("Path ist not valid");

        Content {
            current_path: paths.get(0).unwrap().clone(),
            paths,
            start_path,
            filename,
        }
    }
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Where does this file belong?");
            let name_input =
                ui.add(egui::TextEdit::singleline(&mut self.filename).lock_focus(true));
            /*
            if ctx.input(|i| i.key_pressed(Key::Tab)) {
                if name_input.has_focus() {
                    name_input.request_focus();
                } else {
                    name_input.surrender_focus();
                }
            }
            */
            for p in self.paths.clone() {
                ui.selectable_value(&mut self.current_path, p.clone(), p.name);
            }

            ui.label(format!("{:?}", self.current_path));

            if !name_input.has_focus() {
                if ctx.input(|i| i.key_pressed(Key::J)) {
                    let current_index = self
                        .paths
                        .iter()
                        .position(|p| p.clone() == self.current_path)
                        .unwrap();
                    if current_index + 1 == self.paths.len() {
                        self.current_path = self.paths.get(0).unwrap().clone();
                    } else {
                        self.current_path = self.paths.get(current_index + 1).unwrap().clone();
                    }
                }
                if ctx.input(|i| i.key_pressed(Key::K)) {
                    let current_index = self
                        .paths
                        .iter()
                        .position(|p| p.clone() == self.current_path)
                        .unwrap();
                    if current_index == 0 {
                        self.current_path = self.paths.last().unwrap().clone();
                    } else {
                        self.current_path = self.paths.get(current_index - 1).unwrap().clone();
                    }
                }
                if ctx.input(|i| i.key_pressed(Key::Enter)) {
                    // Move the file
                    let path = mov_file(
                        self.start_path.clone(),
                        self.current_path.path.clone(),
                        self.filename.clone(),
                    );

                    // Open the file
                    let _ = Command::new("xdg-open")
                        .arg(path)
                        .spawn()
                        .expect("Cant Open File");

                    // Close window
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
                if ctx.input(|i| i.key_pressed(Key::Space)) {
                    // Move the file
                    mov_file(
                        self.start_path.clone(),
                        self.current_path.path.clone(),
                        self.filename.clone(),
                    );

                    // Close window
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        });
    }
}
