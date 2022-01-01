use ggez::event;
use ggez::graphics;
use ggez::graphics::MeshBatch;
use ggez::{Context, GameResult};
use hex::World;
use std::env;
use std::path;
use glam::Vec2;
use std::f32::consts::PI;

mod hex;

// First we make a structure to contain the game's state
struct MainState {
    frames: usize,
    meshbatch: MeshBatch,
    world: World,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mb = &mut graphics::MeshBuilder::new();
        let world = hex::World::new();

        //Draw hexagon points clockwise - top left point of hexagon is "origin"
        //size is edge length
        //coordinates are from simple trig

        let h = world.size * (PI/3.0).sin();
        let t = world.size * (PI/6.0).sin();

        mb.line(
            &[
                Vec2::new(0.0,0.0),
                Vec2::new(world.size, 0.0),
                Vec2::new(world.size + t, -h),
                Vec2::new(world.size, -2.0 * h),
                Vec2::new(0.0, -2.0 * h),
                Vec2::new(-t, -h),
                Vec2::new(0.0,0.0),
            ],
            1.0,
            graphics::Color::new(1.0, 1.0, 1.0, 1.0),
        )?;

        let mesh = mb.build(ctx)?;
        let s = MainState { frames: 0, meshbatch: MeshBatch::new(mesh)?, world };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        self.meshbatch.clear();

        //todo only clear meshbatch and recalculate world if something changes
        for (cell, data) in &self.world.map {
            let (x,y) = cell.cartesian_center(self.world.spacing);
            let p = graphics::DrawParam::new().dest(
                Vec2::new(x+2.0 * self.world.radius as f32*self.world.size,y+2.0*self.world.radius as f32*self.world.size));
            self.meshbatch.add(p);
        }

        self.meshbatch.draw(ctx, graphics::DrawParam::default())?;

        graphics::present(ctx)?;

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

    let cb = ggez::ContextBuilder::new("hexxxx", "bcorrigan").add_resource_path(resource_dir)
    .window_mode(ggez::conf::WindowMode::default().dimensions(1400.0, 1300.0));
    let (mut ctx, event_loop) = cb.build()?;
    graphics::set_window_title(&ctx, "Hexagons are the bestagons");

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}


