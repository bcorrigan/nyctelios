use bevy::prelude::*;
use hexgrid::Coordinate;
use std::f32::consts::PI;

mod hex;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1920.0, 1200.0).into(),
                title: "Hexagons are the bestagons".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, update_hexagons)
        .run();
}

#[derive(Component)]
struct HexWorld {
    world: hex::World,
    frames: usize,
}

// First, add a component to store the cell's coordinate
#[derive(Component)]
struct HexCell {
    coordinate: Coordinate,
}

#[derive(Resource)]
struct HexagonMesh(Handle<Mesh>);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2d);

    let world = hex::World::new();

    // Create hexagon mesh
    let h = world.size * (PI / 3.0).sin();
    let t = world.size * (PI / 6.0).sin();
    let margin = 1.0;

    let vertices = [
        [0.0 + margin, 0.0 - margin],
        [world.size - margin, 0.0 - margin],
        [world.size + t - margin, -h],
        [world.size - margin, -2.0 * h + margin],
        [0.0 + margin, -2.0 * h + margin],
        [-t + margin, -h],
        [0.0 + margin, 0.0 - margin],
    ];

    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::default(),
    );

    // Convert the hexagon outline into triangles
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    let mut colors = Vec::new();

    // Center point for triangulation
    let center = [world.size / 2.0, -h];
    positions.push([center[0], center[1], 0.0]);

    // Add vertices and create triangles
    for i in 0..6 {
        positions.push([vertices[i][0], vertices[i][1], 0.0]);
        indices.extend_from_slice(&[0, i as u32 + 1, ((i + 1) % 6 + 1) as u32]);
        colors.push([1.0, 1.0, 1.0, 1.0]);
    }
    colors.push([1.0, 1.0, 1.0, 1.0]); // Center point color

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

    // Store the mesh as a resource
    let mesh_handle = HexagonMesh(meshes.add(mesh));

    // Create all hexagon entities initially
    for (cell, data) in &world.map {
        let (x, y) = cell.cartesian_center(world.spacing);
        let offset_x = -0.0 * world.radius as f32 * world.size;
        let offset_y = -0.0 * world.radius as f32 * world.size;
        let position = Vec2::new(x + offset_x, y + offset_y);

        let color = match data {
            &hex::Type::On(i) if i == 2 => Color::srgb_u8(255, 255, 255),
            &hex::Type::On(_) => Color::srgba(0.8, 0.0, 0.0, 1.0),
            &hex::Type::Off => Color::srgba(0.2, 0.2, 0.2, 1.0),
        };

        commands.spawn((
            HexCell { coordinate: *cell },
            Mesh2d(mesh_handle.0.clone().into()),
            MeshMaterial2d(materials.add(ColorMaterial::from(color))),
            Transform::from_xyz(position.x, position.y, 0.0),
        ));
    }

    // Store the mesh handle as a resource
    commands.insert_resource(mesh_handle);

    // Spawn the world component
    commands.spawn(HexWorld { world, frames: 0 });
}

fn update_hexagons(
    mut hex_world: Query<&mut HexWorld>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut hex_cells: Query<(&HexCell, &mut MeshMaterial2d<ColorMaterial>)>,
) {
    let mut hex_world = hex_world.single_mut();
    hex_world.frames += 1;
    //if hex_world.frames % 10 == 0 {
    hex_world.world.iterate();

    // Update existing hexagons
    for (hex_cell, mut material) in &mut hex_cells {
        if let Some(data) = hex_world.world.map.get(&hex_cell.coordinate) {
            let color = match data {
                &hex::Type::On(i) => Color::srgb_u8(i * 20 + 170, (i * i * i) * 3, 0),
                &hex::Type::Off => Color::srgba(0.2, 0.2, 0.2, 1.0),
            };

            // Update the material
            material.0 = materials.add(ColorMaterial::from(color));
        }
    }

    println!("Frame: {}", hex_world.frames);
    //}
}
