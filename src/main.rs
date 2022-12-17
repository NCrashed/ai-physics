extern crate sdl2;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;
const CIRCLE_RADIUS: f32 = 10.0;

struct FPoint {
    x: f32,
    y: f32,
}

impl FPoint {
    pub fn new(x: f32, y: f32) -> FPoint {
        FPoint { x, y }
    }
}

#[derive(Copy, Clone)]
struct Circle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

impl Circle {
    fn new(x: f32, y: f32) -> Circle {
        Circle {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
        }
    }

    fn update(&mut self) {
        self.x += self.vx;
        self.y += self.vy;

        if self.x < CIRCLE_RADIUS {
            self.x = CIRCLE_RADIUS;
            self.vx = -self.vx;
        }
        if self.x > WINDOW_WIDTH as f32 - CIRCLE_RADIUS {
            self.x = WINDOW_WIDTH as f32 - CIRCLE_RADIUS;
            self.vx = -self.vx;
        }
        if self.y < CIRCLE_RADIUS {
            self.y = CIRCLE_RADIUS;
            self.vy = -self.vy;
        }
        if self.y > WINDOW_HEIGHT as f32 - CIRCLE_RADIUS {
            self.y = WINDOW_HEIGHT as f32 - CIRCLE_RADIUS;
            self.vy = -self.vy;
        }
    }

    fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let circle_color = Color::RGB(0, 0, 255);
        let circle_center = Point::new(self.x as i32, self.y as i32);
        let circle_radius = CIRCLE_RADIUS;
        canvas
            .filled_circle(
                circle_center.x as i16,
                circle_center.y as i16,
                circle_radius as i16,
                circle_color,
            )
            .unwrap();
    }

    fn collides_with(&self, other: &Circle) -> bool {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let distance_squared = dx * dx + dy * dy;
        distance_squared < (CIRCLE_RADIUS * 2.0) * (CIRCLE_RADIUS * 2.0)
    }

    fn bounce_off(&mut self, other: &mut Circle) {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let distance_squared = dx * dx + dy * dy;
        let distance = distance_squared.sqrt();

        let unit_normal = FPoint::new(
            (self.x - other.x) / distance,
            (self.y - other.y) / distance,
        );
        let unit_tangent = FPoint::new(-unit_normal.y, unit_normal.x);

        let self_normal_velocity = self.vx * unit_normal.x + self.vy * unit_normal.y;
        let self_tangent_velocity = self.vx * unit_tangent.x + self.vy * unit_tangent.y;
        let other_normal_velocity = other.vx * unit_normal.x + other.vy * unit_normal.y;
        let other_tangent_velocity = other.vx * unit_tangent.x + other.vy * unit_tangent.y;

        let self_normal_velocity_prime = other_normal_velocity;
        let other_normal_velocity_prime = self_normal_velocity;

        self.vx =
            self_normal_velocity_prime * unit_normal.x + self_tangent_velocity * unit_tangent.x;
        self.vy =
            self_normal_velocity_prime * unit_normal.y + self_tangent_velocity * unit_tangent.y;
        other.vx =
            other_normal_velocity_prime * unit_normal.x + other_tangent_velocity * unit_tangent.x;
        other.vy =
            other_normal_velocity_prime * unit_normal.y + other_tangent_velocity * unit_tangent.y;
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Circle Game", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut player_circle = Circle::new(100.0, 100.0);
    let mut circles = Vec::new();
    for _ in 0..10 {
        circles.push(Circle::new(
            rand::random::<f32>() * WINDOW_WIDTH as f32,
            rand::random::<f32>() * WINDOW_HEIGHT as f32,
        ));
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::W => player_circle.vy -= 1.0,
                    Keycode::A => player_circle.vx -= 1.0,
                    Keycode::S => player_circle.vy += 1.0,
                    Keycode::D => player_circle.vx += 1.0,
                    _ => {}
                },
                _ => {}
            }
        }

        player_circle.update();
        for i in 0..circles.len() {
            circles[i].update();
            if player_circle.collides_with(&circles[i]) {
                player_circle.bounce_off(&mut circles[i]);
            }
            for j in i + 1..circles.len() {
                if circles[i].collides_with(&circles[j]) {
                    let mut other_circle = circles[j];
                    circles[i].bounce_off(&mut other_circle);
                    circles[j] = other_circle;
                }
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        let player_circle_color = Color::RGB(255, 0, 0);
        let player_circle_center = Point::new(player_circle.x as i32, player_circle.y as i32);
        let player_circle_radius = CIRCLE_RADIUS;
        canvas
            .filled_circle(
                player_circle_center.x as i16,
                player_circle_center.y as i16,
                player_circle_radius as i16,
                player_circle_color,
            )
            .unwrap();

        for circle in circles.iter() {
            circle.render(&mut canvas);
        }

        canvas.present();
    }
}
