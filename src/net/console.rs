// [[file:../../bevy.note::6c039888][6c039888]]
// #![deny(warnings)]
#![deny(clippy::all)]

use super::{RemoteCommand, StreamEvent};
use crate::net::ServerPlugin;
use gchemol_core::Molecule;

use bevy::prelude::*;
// 6c039888 ends here

// [[file:../../bevy.note::d66e839e][d66e839e]]
pub struct RemoteConsolePlugin;
// d66e839e ends here

// [[file:../../bevy.note::22cddf8a][22cddf8a]]
fn delete_command(
    mut commands: Commands,
    mut molecule_query: Query<Entity, With<crate::player::Molecule>>,
    mut reader: EventReader<StreamEvent>,
) {
    for (_per_frame, StreamEvent(cmd)) in reader.iter().enumerate() {
        if let RemoteCommand::Delete = cmd {
            if let Ok(molecule_entity) = molecule_query.get_single() {
                info!("remove molecule");
                commands.entity(molecule_entity).despawn_recursive();
            }
        }
    }
}
// 22cddf8a ends here

// [[file:../../bevy.note::3d0c7156][3d0c7156]]
impl Plugin for RemoteConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(delete_command);
        //
    }
}
// 3d0c7156 ends here
