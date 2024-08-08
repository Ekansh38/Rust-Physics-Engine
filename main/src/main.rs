use macroquad::prelude::*;

fn update_all_balls(balls: &mut Vec<Ball>, delta_time: f32, mouse_tregectory: &mut Vec<Vector>) {
    for ball in balls.iter_mut() {
        ball.throwing_logic(mouse_tregectory);
        ball.update(delta_time);
    }
    for i in 0..balls.len() {
        for j in i + 1..balls.len() {
            let (left, right) = balls.split_at_mut(j);
            left[i].collide(&mut right[0]);
        }
    }
}

#[macroquad::main("Physics Engine")]
async fn main() {
    let mut balls = vec![
        Ball::new(500.0, 100.0, 75.0, RED, 0.99, 0.6, 10.8),
        Ball::new(100.0, 100.0, 45.0, YELLOW, 0.99, 0.8, 1.8),
    ];

    let mut mouse_tregectory: Vec<Vector> = Vec::new();

    // Fps Logic
    let mut fps = 0;
    let mut update_fps_counter = 0.0;
    let mut can_update_fps = true;

    let mut previous_time = get_time();

    loop {
        let current_time = get_time();
        let delta_time = (current_time - previous_time) as f32;
        previous_time = current_time;

        clear_background(BLACK);

        update_all_balls(&mut balls, delta_time, &mut mouse_tregectory);

        // Logic for FPS

        draw_text(
            &format!("FPS: {}", fps.to_string()),
            10.0,
            20.0,
            32.0,
            WHITE,
        );

        if can_update_fps {
            fps = get_fps();
            update_fps_counter = 0.0;
            can_update_fps = false;
        } else {
            update_fps_counter += 150.0 * delta_time;
            if update_fps_counter > 100.0 {
                can_update_fps = true;
            }
        }
        next_frame().await
    }
}
#[derive(Clone, Copy)]
struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn dot(&self, other: &Vector) -> f32 {
        self.x * other.x + self.y * other.y
    }

    fn add(&self, other: &Vector) -> Vector {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn subract(&self, other: &Vector) -> Vector {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn divide(&self, other: &Vector) -> Vector {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }

    fn multiply(&self, other: &Vector) -> Vector {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }

    fn dist(&self, other: &Vector) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

struct Ball {
    pos: Vector,
    r: f32,
    c: Color,
    vel: Vector,
    grabing: bool,
    surface_friction: f32,
    retention: f32,
    mass: f32,
    force: Vector,
}

impl Ball {
    fn new(
        x: f32,
        y: f32,
        r: f32,
        c: Color,
        surface_friction: f32,
        retention: f32,
        mass: f32,
    ) -> Self {
        Self {
            pos: Vector::new(x, y),
            r,
            c,
            vel: Vector::new(0.0, 0.0),
            grabing: false,
            surface_friction,
            retention,
            mass,
            force: Vector::new(0.0, 0.0),
        }
    }

    fn friction_checks(&mut self) {
        if self.pos.y + self.r >= screen_height() {
            self.vel.x = self.vel.x * self.surface_friction
        }
    }

    fn euler_integration(&mut self, delta_time: f32) {
        // Newton's second law of motion: F = ma
        let mass_vector = Vector::new(self.mass, self.mass);
        let acc = self.force.divide(&mass_vector);

        // Update velocity using Euler's method
        let delta_time_vector = Vector::new(delta_time, delta_time);
        self.vel = self.vel.add(&acc.multiply(&delta_time_vector));

        self.pos = self.pos.add(&self.vel.multiply(&delta_time_vector));
    }

    fn apply_gravity(&mut self, delta_time: f32) {
        // Apply gravity
        let pixels_per_meter = 200.0;
        let universal_gravity_constant = 9.8; // 9.8 m/s^2
        let gravity = universal_gravity_constant * pixels_per_meter;
        self.vel.y += gravity * delta_time;
    }

    fn edges(&mut self) {
        if self.pos.y + self.r > screen_height() {
            self.pos.y = screen_height() - self.r;
            self.vel.y = self.vel.y * -1.0 * self.retention;
        }

        if self.pos.x + self.r > screen_width() {
            self.pos.x = screen_width() - self.r;
            self.vel.x = self.vel.x * -1.0 * self.retention;
        } else if self.pos.x - self.r < 0.0 {
            self.pos.x = self.r;
            self.vel.x = self.vel.x * -1.0 * self.retention;
        }
    }

    fn throwing_logic(&mut self, mouse_tregectory: &mut Vec<Vector>) {
        let grabing = self.is_grabing();

        if grabing == 1 {
            mouse_tregectory.push(Vector::new(mouse_position().0, mouse_position().1));
            if mouse_tregectory.len() > 20 {
                mouse_tregectory.remove(0);
            }
        } else if grabing == -1 {
            self.vel.x = 0.0;
            self.vel.y = 0.0;
            let push = mouse_tregectory[0].subract(&mouse_tregectory[mouse_tregectory.len() - 1]);

            let reverse_vector = Vector::new(-1.0, -1.0);
            let push = push.multiply(&reverse_vector);

            let mag = Vector::new(2000.0, 2000.0);
            let push = push.multiply(&mag);

            let force = push;
            self.apply_force(force);

            mouse_tregectory.clear();
        }
    }

    fn update(&mut self, delta_time: f32) {
        if !self.grabing {
            self.apply_gravity(delta_time);
            self.friction_checks();
            self.euler_integration(delta_time);
            self.edges();
        } else {
            self.pos.x = mouse_position().0;
            self.pos.y = mouse_position().1;
            self.vel = Vector::new(0.0, 0.0);
        }
        // Update the force to 0
        self.force = Vector::new(0.0, 0.0);

        // Draw the ball
        self.draw();
    }

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r + 2.0, WHITE);
        draw_circle(self.pos.x, self.pos.y, self.r, self.c);
    }

    fn is_grabing(&mut self) -> i32 {
        let mouse_pos = mouse_position();
        if is_mouse_button_pressed(MouseButton::Left)
            && mouse_pos.0 > self.pos.x - self.r
            && mouse_pos.0 < self.pos.x + self.r
            && mouse_pos.1 > self.pos.y - self.r
            && mouse_pos.1 < self.pos.y + self.r
        {
            self.grabing = true;
        } else if is_mouse_button_released(MouseButton::Left) && self.grabing {
            self.grabing = false;
            self.vel.y = 0.0;
            return -1;
        }

        if self.grabing {
            return 1;
        } else {
            return 0;
        }
    }

    fn apply_force(&mut self, force: Vector) {
        self.force = force;
    }

    fn collide(&mut self, other: &mut Ball) {
        let distance = self.pos.dist(&other.pos);
        let sum_radii = self.r + other.r;

        if distance < sum_radii {
            let line_of_impact = other
                .pos
                .subract(&self.pos)
                .divide(&Vector::new(distance, distance));

            let relative_velocity = other.vel.subract(&self.vel);
            let velocity_along_normal = relative_velocity.dot(&line_of_impact);

            if velocity_along_normal > 0.0 {
                return;
            }

            let restitution = 1.0; // Elastic collision
            let impulse_scalar = -(1.0 + restitution) * velocity_along_normal;

            let impulse = line_of_impact.multiply(&Vector::new(impulse_scalar, impulse_scalar));

            self.vel = self
                .vel
                .subract(&impulse.divide(&Vector::new(self.mass, self.mass)));
            other.vel = other
                .vel
                .add(&impulse.divide(&Vector::new(other.mass, other.mass)));
        }
    }
}
