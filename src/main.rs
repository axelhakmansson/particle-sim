use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::*;



struct Particle {
    pos: Vec2,
    acc: Vec2,
    vel: Vec2
}

impl Particle {
    fn new(pos: Vec2, acc: Vec2, vel: Vec2) -> GameResult<Particle> {
        let s = Particle { pos, acc, vel };
        Ok(s)
    }
    fn gravity(&mut self) {
        self.acc = Vec2 { x: 0.0, y: 0.982 };
    }
}

impl event::EventHandler<ggez::GameError> for Particle {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.vel += self.acc;
        self.pos += self.vel;
        self.acc *= 0.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::WHITE,
        );

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::ZERO,
            10.0,
            2.0,
            Color::BLACK,
        )?;
        canvas.draw(&circle, self.pos);

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let mut particle = Particle::new(Vec2 { x: 200.0, y: 30.0 }, Vec2::ZERO, Vec2::ZERO)?;
    particle.gravity();
    event::run(ctx, event_loop, particle)
}