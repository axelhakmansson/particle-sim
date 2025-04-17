use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::*;
use rand::prelude::*;

const HEIGHT: f32 = 600.0;
const WIDTH: f32 = 800.0;
const PARTICLE_RADIUS: f32 = 10.0;

struct Particle {
    pos: Vec2,
    acc: Vec2,
    vel: Vec2
}

impl Particle {
    fn new(pos: Vec2, acc: Vec2, vel: Vec2) -> GameResult<Particle> {
        let particle = Particle { pos, acc, vel };
        Ok(particle)
    }
    fn gravity(&mut self) {
        self.acc = Vec2 { x: 0.0, y: 0.982 };
    }
}

struct MainState {
    particles: Vec<Particle>
}

impl MainState {
    fn new(num_of_particles: u32) -> GameResult<MainState> {
        let mut rng = rand::rng();
        let mut particles = vec![];
        for _ in 0..num_of_particles {
            particles.push(Particle { 
                pos: Vec2 { x: rng.random_range(0.0..WIDTH), y: rng.random_range(0.0..HEIGHT) }, 
                acc: Vec2 { x: rng.random_range(-1.0..1.0), y: rng.random_range(-1.0..1.0) }, 
                vel: Vec2 { x: rng.random_range(-1.0..1.0), y: rng.random_range(-1.0..1.0) } });
        }
        let state = MainState { particles };
        Ok(state)
    }
}


impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for particle in &mut self.particles {
            particle.vel += particle.acc;
            particle.pos += particle.vel;
            particle.acc *= 0.0;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::WHITE,
        );

        for particle in &self.particles {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::ZERO,
                PARTICLE_RADIUS,
                2.0,
                Color::BLACK,
            )?;
            canvas.draw(&circle, particle.pos);
        }
        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let mut state = MainState::new(10)?;
    event::run(ctx, event_loop, state)
}