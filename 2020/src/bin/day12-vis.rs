use anyhow::{anyhow, Result};
use bevy::prelude::*;

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

struct GameState {
    instr: Vec<Instr>,
    idx: usize,
    timer: Timer,
}

struct Ferry;
struct Waypoint;
struct Day12Vis;
struct QueuedMovement {
    destination: Vec2,
    speed: f32,
}

impl Plugin for Day12Vis {
    fn build(&self, app: &mut AppBuilder) {
        let input = include_str!("../../input/day12.txt");
        let instr = input
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<Instr>>>()
            .unwrap();

        app.add_resource(WindowDescriptor {
            title: "Advent of Code 2020 Day 12".to_string(),
            ..Default::default()
        })
        .add_resource(GameState { instr, idx: 0, timer: Timer::from_seconds(1., false) })
        .add_resource(ClearColor(Color::rgb(0., 63. / 256., 102. / 256.)))
        .add_startup_system(setup.system())
        .add_system(interpretor.system())
        .add_system(queued_movement.system());
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ferry = asset_server.load("textures/ferry.png");

    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: materials.add(ferry.into()),
            ..Default::default()
        })
        .with(Ferry);
}

fn interpretor(time: Res<Time>, mut gs: ResMut<GameState>, ferry_query: Query<(&Ferry, &Transform)>, mut waypoint_query: Query<(&Waypoint, &mut Transform)>) {
    gs.timer.tick(time.delta_seconds());
    if !gs.timer.finished() {
        return
    }

    // TODO: Check if animations have finished
    match gs.instr[gs.idx] {
        Instr::N(v) => waypoint_query.iter_mut().for_each(|(_, mut t)| {t.translation += Vec3::new(0., v, 0.)}),
        _ => panic!("oops"),
    }
}

fn queued_movement(time: Res<Time>, mut query: Query<(&QueuedMovement, &mut Transform)>) {
    for (m, mut trans) in query.iter_mut() {
        trans.translation.x += time.delta_seconds() * m.speed;
        trans.translation.y += time.delta_seconds() * m.speed;
    }
}

fn main() {
    println!("{:?}", std::env::current_exe());
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(Day12Vis)
        .run();
}
