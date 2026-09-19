use bevy::prelude::App;
use bevy::prelude::Cuboid;
use bevy::prelude::*;
use nannou::NannouPlugin;
use nannou::prelude::*;

#[derive(Resource)]
struct TextureTarget(RenderTexture);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, NannouPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_texture, rotate_cube))
        .run();
}

fn setup(
    app: nannou::context::App,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let target = app.new_render_texture(256, 256);
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(target.image().clone()),
        ..default()
    });

    commands.insert_resource(TextureTarget(target));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.5, 2.5, 2.5))),
        MeshMaterial3d(material),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(4.0, 3.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            intensity: 1_500_000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 5.0, 4.0),
    ));
}

fn draw_texture(mut target: ResMut<TextureTarget>, app: nannou::context::App) {
    if let Some(draw) = target.0.draw(&app) {
        draw.background().color(DARK_GRAY);
        draw.rect()
            .w_h(160.0, 160.0)
            .color(RED)
            .stroke(WHITE)
            .stroke_weight(6.0)
            .rotate(app.time());
        draw.ellipse()
            .w_h(120.0, 80.0)
            .color(BLUE)
            .stroke(WHITE)
            .stroke_weight(6.0)
            .rotate(-app.time());
    }
}

fn rotate_cube(time: Res<Time>, mut cubes: Query<&mut Transform, With<Mesh3d>>) {
    for mut transform in &mut cubes {
        transform.rotation = Quat::from_rotation_y(time.elapsed_secs() * 0.7)
            * Quat::from_rotation_x(time.elapsed_secs() * 0.4);
    }
}
