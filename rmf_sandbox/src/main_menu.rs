use super::demo_world::demo_office;
use bevy::{app::AppExit, prelude::*, tasks::AsyncComputeTaskPool};
use bevy_egui::{egui, EguiContext};

use crate::AppState;
use crate::{building_map::BuildingMap, OpenedMapFile};

use {bevy::tasks::Task, futures_lite::future, rfd::AsyncFileDialog};

struct LoadMapResult(Option<OpenedMapFile>, BuildingMap);

#[derive(Component)]
struct LoadMapTask(Task<Option<LoadMapResult>>);

fn egui_ui(
    mut egui_context: ResMut<EguiContext>,
    mut _commands: Commands,
    mut _exit: EventWriter<AppExit>,
    mut app_state: ResMut<State<AppState>>,
) {
    egui::Window::new("Welcome!")
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0., 0.))
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("Welcome to The RMF Sandbox!");
            ui.add_space(10.);

            ui.horizontal(|ui| {
                if ui.button("View demo map").clicked() {
                    // load the office demo that is hard-coded in demo_world.rs
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let future = AsyncComputeTaskPool::get().spawn(async move {
                            let yaml = demo_office();
                            let data = yaml.as_bytes();
                            let map = match BuildingMap::from_bytes(&data) {
                                Ok(map) => map,
                                Err(err) => {
                                    println!("{:?}", err);
                                    return None;
                                }
                            };
                            Some(LoadMapResult(None, map))
                        });
                        _commands.spawn().insert(LoadMapTask(future));
                    }

                    // on web, we don't have a handy thread pool, so we'll
                    // just parse the map here in the main thread.
                    #[cfg(target_arch = "wasm32")]
                    {
                        let yaml = demo_office();
                        let data = yaml.as_bytes();
                        match BuildingMap::from_bytes(&data) {
                            Ok(map) => {
                                _commands.insert_resource(map);
                                match app_state.set(AppState::TrafficEditor) {
                                    Ok(_) => {}
                                    Err(err) => {
                                        println!("Failed to enter traffic editor: {:?}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                println!("{:?}", err);
                            }
                        }
                    }

                    // switch to using a channel to signal completing the task
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    if ui.button("Open a map file").clicked() {
                        // load the map in a thread pool
                        let future = AsyncComputeTaskPool::get().spawn(async move {
                            let file = match AsyncFileDialog::new().pick_file().await {
                                Some(file) => file,
                                None => {
                                    println!("No file selected");
                                    return None;
                                }
                            };
                            println!("Loading site map");
                            let data = file.read().await;
                            let map = match BuildingMap::from_bytes(&data) {
                                Ok(map) => map,
                                Err(err) => {
                                    println!("{:?}", err);
                                    return None;
                                }
                            };
                            Some(LoadMapResult(
                                Some(OpenedMapFile(file.path().to_path_buf())),
                                map,
                            ))
                        });
                        _commands.spawn().insert(LoadMapTask(future));
                    }
                }

                if ui.button("Warehouse generator").clicked() {
                    println!("Entering warehouse generator");
                    app_state.set(AppState::WarehouseGenerator).unwrap();
                }
            });

            #[cfg(not(target_arch = "wasm32"))]
            {
                ui.add_space(20.);
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        if ui.button("Exit").clicked() {
                            _exit.send(AppExit);
                        }
                    });
                });
            }
        });
}

/// Handles the file opening events
#[cfg(not(target_arch = "wasm32"))]
fn map_load_complete(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut LoadMapTask)>,
    mut app_state: ResMut<State<AppState>>,
) {
    for (entity, mut task) in tasks.iter_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            println!("Site map loaded");
            commands.entity(entity).despawn();

            match result {
                Some(result) => {
                    println!("Entering traffic editor");
                    commands.insert_resource(result.0);
                    commands.insert_resource(result.1);
                    match app_state.set(AppState::TrafficEditor) {
                        Ok(_) => {}
                        Err(err) => {
                            println!("Failed to enter traffic editor: {:?}", err);
                        }
                    }
                }
                None => {}
            }
        }
    }
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(egui_ui));

        #[cfg(not(target_arch = "wasm32"))]
        app.add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(map_load_complete));
    }
}
