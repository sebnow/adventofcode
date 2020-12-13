use anyhow::{anyhow, Result};
use bevy::prelude::*;

const CELL_SIZE: f32 = 24.;
const FERRY_SPEED: f32 = 10.0 * CELL_SIZE;
const WAYPOINT_SPEED: f32 = FERRY_SPEED;

#[derive(Debug, Clone, Copy)]
enum Instr {
    N(f32),
    S(f32),
    E(f32),
    W(f32),
    L(f32),
    R(f32),
    F(f32),
}

impl std::str::FromStr for Instr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: f32 = s[1..].parse()?;
        match s.chars().nth(0) {
            Some('N') => Ok(Instr::N(v)),
            Some('S') => Ok(Instr::S(v)),
            Some('E') => Ok(Instr::E(v)),
            Some('W') => Ok(Instr::W(v)),
            Some('L') => Ok(Instr::L(v)),
            Some('R') => Ok(Instr::R(v)),
            Some('F') => Ok(Instr::F(v)),
            _ => Err(anyhow!("invalid instruction {}", s)),
        }
    }
}

impl std::fmt::Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Instr::N(v) => format!("N{}", v),
                Instr::S(v) => format!("S{}", v),
                Instr::E(v) => format!("E{}", v),
                Instr::W(v) => format!("W{}", v),
                Instr::L(v) => format!("L{}", v),
                Instr::R(v) => format!("R{}", v),
                Instr::F(v) => format!("F{}", v),
            }
        )
    }
}
struct GameState {
    instruction: Instr,
    instructions: Vec<Instr>,
    idx: usize,
    timer: Timer,
}

struct Ferry;
struct Waypoint;
struct Day12Vis;
struct TargetDestination(Vec3);
struct Movable(f32);
struct InstructionText;

struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init_instruction_text.system())
            .add_system(update_instruction_text);
    }
}

fn init_instruction_text(commands: &mut Commands, asset_server: ResMut<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Regular.ttf");

    commands
        .spawn(CameraUiBundle::default())
        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(CELL_SIZE),
                    top: Val::Px(CELL_SIZE),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                value: "Instruction: ".to_string(),
                font,
                style: TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.8, 0.8, 0.8),
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(InstructionText);
}

fn update_instruction_text(gs: Res<GameState>, mut query: Query<(&mut Text, &InstructionText)>) {
    for (mut text, _) in query.iter_mut() {
        text.value = format!("Instruction: {}", gs.instruction);
    }
}

impl Plugin for Day12Vis {
    fn build(&self, app: &mut AppBuilder) {
        let input = include_str!("../../input/day12.txt");
        let instr = input
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<Instr>>>()
            .unwrap();

        let mut timer = Timer::from_seconds(0.1, true);
        timer.pause();

        app.add_resource(WindowDescriptor {
            title: "Advent of Code 2020 Day 12".to_string(),
            ..Default::default()
        })
        .add_resource(GameState {
            instruction: instr[0],
            instructions: instr,
            idx: 0,
            timer,
        })
        .add_resource(ClearColor(Color::rgb(0., 63. / 256., 102. / 256.)))
        .add_startup_system(setup.system())
        .add_system(inputs.system())
        .add_system(interpretor.system())
        .add_system(queued_movement.system())
        .add_system(ferry_rotation.system());
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ferry = asset_server.load("textures/ferry.png");
    let waypoint = asset_server.load("textures/waypoint.png");
    let waypoint_loc = Vec3::new(10., 1., 0.) * CELL_SIZE;
    let ferry_angle = std::f32::consts::PI / 2.0;

    commands
        .spawn(Camera2dBundle::default())
        .spawn((
            Ferry,
            TargetDestination(Vec3::new(0., 0., 0.)),
            Movable(FERRY_SPEED),
        ))
        .with_bundle(SpriteBundle {
            material: materials.add(ferry.into()),
            transform: Transform {
                // Pointing east
                rotation: Quat::from_rotation_z(ferry_angle),
                ..Default::default()
            },
            ..Default::default()
        })
        .spawn((
            Waypoint,
            TargetDestination(waypoint_loc),
            Movable(WAYPOINT_SPEED),
        ))
        .with_bundle(SpriteBundle {
            material: materials.add(waypoint.into()),
            transform: Transform {
                translation: waypoint_loc,
                ..Default::default()
            },
            ..Default::default()
        });
}

fn interpretor(
    time: Res<Time>,
    mut gs: ResMut<GameState>,
    mut ferry_query: Query<(&Ferry, &Transform, &mut TargetDestination)>,
    mut waypoint_query: Query<(&Waypoint, &Transform, &mut TargetDestination)>,
) {
    gs.timer.tick(time.delta_seconds());
    if !gs.timer.finished() {
        return;
    }

    // Awkward but whatever
    for (_, ferry_transform, mut ferry_target) in ferry_query.iter_mut() {
        let ferry_dest = &mut ferry_target.0;
        for (_, wp_transform, mut wp_target) in waypoint_query.iter_mut() {
            let wp_dest = &mut wp_target.0;

            // Check if animations have finished
            if wp_transform.translation.distance(*wp_dest) > 0.1
                || ferry_transform.translation.distance(*ferry_dest) > 0.1
            {
                return;
            }

            let relative = wp_transform.translation - ferry_transform.translation;

            gs.instruction = gs.instructions[gs.idx];
            match gs.instruction {
                Instr::N(v) => wp_dest.y += v * CELL_SIZE,
                Instr::S(v) => wp_dest.y -= v * CELL_SIZE,
                Instr::E(v) => wp_dest.x += v * CELL_SIZE,
                Instr::W(v) => wp_dest.x -= v * CELL_SIZE,
                Instr::L(v) => {
                    *wp_dest = (0..(v / 90.0).ceil() as usize)
                        .fold(relative, |r, _| Vec3::new(r.y * -1.0, r.x, r.z));
                }
                Instr::R(v) => {
                    *wp_dest = (0..(v / 90.0).ceil() as usize)
                        .fold(relative, |r, _| Vec3::new(r.y, r.x - 1., r.z));
                }
                Instr::F(v) => {
                    //*ferry_dest += relative * v * CELL_SIZE;
                    //*wp_dest += relative * v * CELL_SIZE;
                }
            }

            println!("Waypoint {:?} -> {:?}", wp_transform.translation, wp_dest);
            println!(
                "---> Waypoint is {} away from target",
                wp_transform.translation.distance(*wp_dest)
            );
            println!(
                "Ferry {:?} -> {:?}",
                ferry_transform.translation, ferry_dest
            );
            println!(
                "---> Ferry is {} away from target",
                ferry_transform.translation.distance(*ferry_dest)
            );
        }
    }

    gs.idx += 1;
}

fn queued_movement(
    time: Res<Time>,
    mut query: Query<(&TargetDestination, &Movable, &mut Transform)>,
) {
    for (target, movable, mut trans) in query.iter_mut() {
        let distance_delta = movable.0 * time.delta_seconds();
        trans.translation = move_toward(trans.translation, target.0, distance_delta);
    }
}

fn ferry_rotation(
    time: Res<Time>,
    mut ferry_query: Query<(&Ferry, &mut Transform)>,
    waypoint_query: Query<(&Waypoint, &Transform)>,
) {
//    for (_, mut transform) in ferry_query.iter_mut() {
//        for (_, target) in waypoint_query.iter() {
//            let theta = transform.translation.angle_between(target.translation);
//            println!("Angle to waypoint is {}", theta);
//            //transform.rotate(Quat::from_rotation_z(theta * time.delta_seconds()));
//        }
//    }
}

fn inputs(mut gs: ResMut<GameState>, key_input: Res<Input<KeyCode>>) {
    if key_input.pressed(KeyCode::Space) {
        if gs.timer.paused() {
            gs.timer.unpause();
        } else {
            gs.timer.pause();
        }
    }
}

fn move_toward(origin: Vec3, target: Vec3, distance_delta: f32) -> Vec3 {
    let diff = target - origin;
    let magnitude = diff.length();
    if magnitude <= distance_delta || magnitude == 0. {
        target
    } else {
        origin + diff / magnitude * distance_delta
    }
}

fn main() {
    println!("{:?}", std::env::current_exe());
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(Day12Vis)
        .add_plugin(UIPlugin)
        .run();
}
