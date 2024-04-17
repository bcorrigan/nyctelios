use ggez::event;
use ggez::graphics;
use ggez::graphics::{Color, Mesh};
use ggez::graphics::{DrawMode, FillOptions, InstanceArray};
use ggez::{Context, GameResult};
use glam::Vec2;
use hex::World;
use std::env;
use std::f32::consts::PI;
use std::path;

mod hex;

// First we make a structure to contain the game's state
struct MainState {
    frames: usize,
    meshbatch: InstanceArray,
    mesh: Mesh,
    world: World,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mb = &mut graphics::MeshBuilder::new();
        let world = hex::World::new();

        //Draw hexagon points clockwise - top left point of hexagon is "origin"
        //size is edge length
        //coordinates are from simple trig

        let h = world.size * (PI / 3.0).sin();
        let t = world.size * (PI / 6.0).sin();
        let margin = 1.0;

        mb.polygon(
            DrawMode::Fill(FillOptions::default()),
            &[
                Vec2::new(0.0 + margin, 0.0 - margin),
                Vec2::new(world.size - margin, 0.0 - margin),
                Vec2::new(world.size + t - margin, -h),
                Vec2::new(world.size - margin, -2.0 * h + margin),
                Vec2::new(0.0 + margin, -2.0 * h + margin),
                Vec2::new(-t + margin, -h),
                Vec2::new(0.0 + margin, 0.0 - margin),
            ],
            graphics::Color::new(1.0, 1.0, 1.0, 1.0),
        )?;

        let meshbatch = InstanceArray::new(ctx, None);

        //let instancearray = InstanceArray::new(&mb.build(ctx), None);
        let s = MainState {
            frames: 0,
            meshbatch,
            mesh: Mesh::from_data(ctx, mb.build()),
            world,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        self.meshbatch.clear();

        //if (self.frames % 25) == 24 {
        self.world.iterate();
        //}

        //todo only clear meshbatch and recalculate world if something changes
        for (cell, data) in &self.world.map {
            let (x, y) = cell.cartesian_center(self.world.spacing);
            let p = graphics::DrawParam::new().dest(Vec2::new(
                x + 2.0 * self.world.radius as f32 * self.world.size,
                y + 2.0 * self.world.radius as f32 * self.world.size,
            ));
            let p2 = match data {
                &hex::Type::On(i) => {
                    if i == 2 {
                        //
                        p.color(graphics::Color::new(1.0, 1.0, 0.0, 1.0))
                    } else {
                        p.color(graphics::Color::new(0.8, 0.0, 0.0, 1.0))
                    }
                } //pink
                &hex::Type::Off => p.color(graphics::Color::new(0.2, 0.2, 0.2, 1.0)), //yellow
            };

            self.meshbatch.push(p2);
        }

        //self.meshbatch.draw(ctx, graphics::DrawParam::default())?;
        //canvas.draw(&self.meshbatch, graphics::DrawParam::default());

        canvas.draw_instanced_mesh(
            self.mesh.clone(),
            &self.meshbatch,
            graphics::DrawParam::default(),
        );

        //graphics::present(ctx)?;
        canvas.finish(ctx)?;
        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
        }

        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let mut wm = ggez::conf::WindowMode::default()
        .dimensions(1920.0, 1200.0)
        //.fullscreen_type(ggez::conf::FullscreenType::True)
        .resizable(true)
        .resize_on_scale_factor_change(true);
    wm.logical_size = None;

    let cb = ggez::ContextBuilder::new("hexxxx", "bcorrigan")
        .add_resource_path(resource_dir)
        .window_mode(wm);
    let (mut ctx, event_loop) = cb.build()?;
    graphics::set_window_title(&ctx, "Hexagons are the bestagons");

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
