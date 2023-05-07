// [[file:../bevy.note::a83ae206][a83ae206]]
// #![deny(warnings)]

use crate::player::*;

use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, PickingCameraBundle};
// a83ae206 ends here

// [[file:../bevy.note::031857dd][031857dd]]
use crate::player::FrameIndex;
// 031857dd ends here

// [[file:../bevy.note::711fbcb5][711fbcb5]]
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn update_light_with_camera(
    mut param_set: ParamSet<(
        Query<(&mut Transform, With<DirectionalLight>)>,
        Query<&Transform, With<PanOrbitCamera>>,
    )>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if let Ok(camera) = param_set.p1().get_single() {
        let camera_position = camera.translation;
        for (mut transform, _mesh) in param_set.p0().iter_mut() {
            let distance = transform.translation.distance(camera_position);
            transform.translation = camera_position;
        }
    }

    if let Ok((camera, camera_transform)) = camera_query.get_single() {
        let viewport = camera.world_to_viewport(camera_transform, Vec3::new(2.144404, 2.2027268, 2.6483808));
    }
}
// 711fbcb5 ends here

// [[file:../bevy.note::b93672bb][b93672bb]]
fn setup_lights(commands: &mut Commands) {
    // light
    // ambient light
    let illuminance = 5500.0;
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.20,
    });
    let trans = Transform::from_xyz(5., 5., 5.);
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance,
            ..default()
        },
        transform: trans.looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    let trans = Transform::from_xyz(-5., 5., -5.);
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance,
            ..default()
        },
        transform: trans.looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    let trans = Transform::from_xyz(5., 5., -5.);
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance,
            ..default()
        },
        transform: trans.looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    let trans = Transform::from_xyz(5., -5., -5.);
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance,
            ..default()
        },
        transform: trans.looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
// b93672bb ends here

// [[file:../bevy.note::c068ff9c][c068ff9c]]
#[derive(Resource, Clone, Debug, Default)]
pub struct MoleculeTrajectory {
    mols: Vec<gchemol_core::Molecule>,
}

/// Visilization state
#[derive(Resource, Clone, Debug, Default)]
pub struct VisilizationState {
    pub display_label: bool,
}

impl MoleculeTrajectory {
    pub fn save_as(&self, path: &std::path::Path) {
        use gchemol::io::prelude::*;

        if let Err(err) = gchemol::io::write(path, &self.mols) {
            error!("Write molecules error: {err:?}");
        }
    }
}
// c068ff9c ends here

// [[file:../bevy.note::20198b2d][20198b2d]]
fn keyboard_animation_control(keyboard_input: Res<Input<KeyCode>>, mut current_frame: ResMut<CurrentFrame>) {
    if keyboard_input.just_pressed(KeyCode::Right) {
        current_frame.0 += 1;
    } else if keyboard_input.just_pressed(KeyCode::Left) {
        current_frame.0 -= 1;
    }
}

fn traj_animation_player(
    traj: Res<MoleculeTrajectory>,
    current_frame: Res<CurrentFrame>,
    vis_state: Res<VisilizationState>,
    mut visibility_query: Query<(&mut Visibility, &FrameIndex)>,
) {
    let nframe = traj.mols.len() as isize;
    // % operator not work for negative number. We need Euclidean division.
    // https://users.rust-lang.org/t/why-works-differently-between-rust-and-python/83911
    let ci = current_frame.0.rem_euclid(nframe);
    for (mut visibility, FrameIndex(fi)) in visibility_query.iter_mut() {
        if *fi == ci as usize {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
// 20198b2d ends here

// [[file:../bevy.note::1c6c0570][1c6c0570]]
#[derive(Resource, Clone, Debug, Default)]
struct CurrentFrame(isize);

pub fn spawn_molecules(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animations: ResMut<Assets<AnimationClip>>,
    asset_server: Res<AssetServer>,
    traj: Res<MoleculeTrajectory>,
) {
    // light
    // ambient light
    setup_lights(&mut commands);

    let center = traj
        .mols
        .iter()
        .next()
        .map(|mol| mol.center_of_geometry())
        .unwrap_or_default()
        .map(|x| x as f32);
    let arcball_camera = PanOrbitCamera {
        focus: center.into(),
        allow_upside_down: true,
        enabled: true,
        ..default()
    };

    // mouse: zoom, rotate and translate
    commands
        .spawn(Camera3dBundle {
            // projection: Projection::Orthographic(OrthographicProjection {
            //     near: -500.0,
            //     far: 500.0,
            //     ..default()
            // }),
            ..default()
        })
        .insert(arcball_camera)
        .insert(PickingCameraBundle::default());

    // create atoms and bonds
    for (fi, mol) in traj.mols.iter().enumerate() {
        // only show the first frame
        let visible = fi == 0;
        crate::player::spawn_molecule(mol, visible, fi, &mut commands, &mut meshes, &mut materials);
    }
}
// 1c6c0570 ends here

// [[file:../bevy.note::8ec82258][8ec82258]]
#[derive(Debug, Clone)]
pub struct MoleculePlugin {
    traj: MoleculeTrajectory,
}

impl MoleculePlugin {
    /// Create animation from a vec of molecules
    pub fn from_mols(mols: Vec<gchemol_core::Molecule>) -> Self {
        Self {
            traj: MoleculeTrajectory { mols },
        }
    }
}

impl Plugin for MoleculePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.traj.clone())
            .insert_resource(CurrentFrame(0))
            .insert_resource(VisilizationState::default())
            .add_startup_system(spawn_molecules)
            .add_system(keyboard_animation_control)
            .add_system(traj_animation_player)
            .add_system(update_light_with_camera);

        match self.traj.mols.len() {
            0 | 1 => {}
            _ => {
                // for animation
                // .add_system(frame_control.after(update_atom_labels_with_camera))
                // .add_system(create_atom_label)
                // .add_system(play_animation.in_base_set(PostStartup));
                // .add_system(play_animation.after(update_atom_labels_with_camera));
            }
        }
    }
}
// 8ec82258 ends here
