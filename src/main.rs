//configuration de base

mod logic;
mod player;
mod ui;
use bevy::prelude::*;

// --- CONTRAT ENTRE VOUS : LES ETATS ---
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Exploring,
    Interrogating,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Charge la fenêtre, le son, les images
        .init_state::<GameState>() // Gère le passage Exploration <-> Dialogue
        .add_systems(Startup, setup)
        // C'est ici que vous ajouterez vos futurs systèmes :
        // .add_systems(Update, move_player_system)      <- Étudiant A
        // .add_systems(Update, check_interaction_system) <- Étudiant B
        // .add_systems(Update, update_ui_system)        <- Étudiant C
        .run();
}

fn setup(mut commands: Commands) {
    // On spawn une caméra pour voir quelque chose
    commands.spawn(Camera2dBundle::default());

    println!("Le projet est prêt !");
}
