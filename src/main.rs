use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::*;
use rand::prelude::*;

const HEIGHT: f32 = 600.0;
const WIDTH: f32 = 800.0;
const VEL_RANGE: f32 = 0.5;
const ACC_RANGE: f32 = 0.5;
const PARTICLE_MASS_HIGHER: f32 = 4.0;
const PARTICLE_MASS_LOWER: f32 = 2.0;

#[derive(PartialEq)]
struct Particle {
    pos: Vec2,
    acc: Vec2,
    vel: Vec2,
    radius: f32,
    mass: f32,
}

impl Particle {
    fn new() -> GameResult<Particle> {
        let mut rng = rand::rng();
        let mass = rng.random_range(PARTICLE_MASS_LOWER..PARTICLE_MASS_HIGHER);
        Ok(Particle { 
            pos: Vec2 { x: rng.random_range(0.0..WIDTH), y: rng.random_range(0.0..HEIGHT) }, 
            acc: Vec2 { x: rng.random_range(-ACC_RANGE..ACC_RANGE), y: rng.random_range(-ACC_RANGE..ACC_RANGE) }, 
            vel: Vec2 { x: rng.random_range(-VEL_RANGE..VEL_RANGE), y: rng.random_range(-VEL_RANGE..VEL_RANGE) },
            radius: 3.0 * mass,
            mass: mass,
        })
    }

    fn check_edge_collision(&mut self) {
        if self.pos.x + self.radius > WIDTH {
            self.pos.x = WIDTH - self.radius;
            self.vel.x *= -1.0;
        } else if self.pos.x - self.radius < 0.0 {
            self.pos.x = self.radius;
            self.vel.x *= -1.0;
        }

        if self.pos.y + self.radius > HEIGHT {
            self.pos.y = HEIGHT - self.radius;
            self.vel.y *= -1.0;
        } else if self.pos.y - self.radius < 0.0 {
            self.pos.y = self.radius;
            self.vel.y *= -1.0;
        }
    }
    fn check_particle_collision(&mut self, particles: Vec<&mut Particle>) {
        for particle in particles {
            if self != particle {
                let distance = self.pos.distance(particle.pos);
                let tolerance = self.radius + particle.radius;

                if distance <= tolerance {
                    let normal = (particle.pos - self.pos).normalize();
                    let rel_vel = particle.vel - self.vel;
                    let impulse = normal * (2.0 * rel_vel.dot(normal)) / (self.mass + particle.mass);

                    let repulsion = normal * (tolerance - distance);

                    self.vel += impulse / self.mass;
                    particle.vel -= impulse / particle.mass;

                    self.pos -= repulsion / self.mass;
                    particle.pos += repulsion / particle.mass;
                }
            }
        }
    }
}

struct MainState {
    particles: Vec<Particle>
}

impl MainState {
    fn new(num_of_particles: u32) -> GameResult<MainState> {
        let mut particles = vec![];
        for _ in 0..num_of_particles {
            particles.push(Particle::new()?);
        }
        let state = MainState { particles };
        Ok(state)
    }
}


impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for i in 0..self.particles.len() {
            let (left, right) = self.particles.split_at_mut(i);
            let (particle, rest) = right.split_first_mut().unwrap();
            let combined_particles: Vec<_> = left.iter_mut().chain(rest.iter_mut()).collect();
            particle.check_particle_collision(combined_particles);
            particle.check_edge_collision();
            particle.vel += particle.acc;
            particle.pos += particle.vel;
            particle.acc *= 0.0;
            particle.vel *= 1.0;
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
                particle.radius,
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
    let state = MainState::new(50)?;
    event::run(ctx, event_loop, state)
}