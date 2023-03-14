

use ggez::{
    event,
    glam::*,
    graphics::*,
    Context, GameResult, mint::Point2, winit::event::VirtualKeyCode,
};
use legion::World;
use collision::collide_rec_aabb;


pub mod components;
pub mod collision;

pub struct Player {
    pub mesh: Mesh,
    pub pos: Vec2,
}

pub struct Bullet {
    pub mesh: Mesh,
    pub pos: Vec2,
}

struct MainState { 
    score: u32,

    // player
    player: Vec2,
    player_mesh: Mesh,
    
    // bullets
    bullets: Vec<Vec2>,
    bullet_mesh: Mesh,

    // enemies
    enemies: Vec<Vec2>,
    enemy_mesh: Mesh,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {
            score: 0,

            player: vec2(100., 100.),
            player_mesh: Mesh::new_polygon(
                ctx,
                DrawMode::fill(),
                &[vec2(0., 0.), vec2(25., -50.), vec2(50., 0.)],
                Color::WHITE,
            )?,

            bullets: Vec::new(),
            bullet_mesh: Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                vec2(0., 0.),
                10., 
                2.0,
                Color::WHITE
            )?,

            enemies: Vec::from([vec2(100., 100.)]),
            enemy_mesh: Mesh::new_polygon(
                ctx,
                DrawMode::fill(),
                &[vec2(0., 0.), vec2(25., -50.), vec2(50., 0.)],
                Color::RED,
            )?,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // update player position
        if ctx.keyboard.is_key_pressed(VirtualKeyCode::W) {
            self.player.y -= 10.;
        }
        if ctx.keyboard.is_key_pressed(VirtualKeyCode::A) {
            self.player.x -= 10.;
        }
        if ctx.keyboard.is_key_pressed(VirtualKeyCode::S) {
            self.player.y += 10.;
        }
        if ctx.keyboard.is_key_pressed(VirtualKeyCode::D) {
            self.player.x += 10.;
        }

        // shoot
        if ctx.keyboard.is_key_just_pressed(VirtualKeyCode::Space) {
            self.bullets.push(self.player);
        }

        // move bullets
        for bullet in &mut self.bullets {
            *bullet += vec2(0., -25.);
        }

        // delete bullets that have exited the arena
        self.bullets.retain(|&bullet| bullet.y > -10.);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.draw(&self.player_mesh, self.player);
        
        for bullet in &self.bullets {
            canvas.draw(&self.bullet_mesh, *bullet);
        }

        for enemy in &self.enemies {
            canvas.draw(&self.enemy_mesh, *enemy);
        }

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
