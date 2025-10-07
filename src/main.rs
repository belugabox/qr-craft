//! Point d'entrée de l'application QR Craft

use dioxus::launch;

mod components;
mod config;
mod models;
mod services;
mod theme;

use components::app::App;

fn main() {
    launch(App);
}
