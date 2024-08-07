use macroquad::prelude::*;

#[macroquad::main("Physics Engine")]
async fn main() {
    let mut balls = vec![
        Ball::new(100.0, 100.0, 75.0, RED, 0.7, 0.6, 2.8),
        Ball::new(100.0, 100.0, 45.0, YELLOW, 0.8, 0.8, 1.8),
    ];

    let mut mouse_tregectory: Vec<(f32, f32)> = Vec::new();

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

        for ball in balls.iter_mut() {
            ball.throwing_logic(&mut mouse_tregectory);
            ball.update(delta_time);
        }

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

struct Ball {
    x: f32,
    y: f32,
    r: f32,
    c: Color,
    y_vel: f32,
    x_vel: f32,
    grabing: bool,
    surface_friction: f32,
    retention: f32,
    stop_boucing: f32,
    mass: f32,
    force: (f32, f32),
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
            x,
            y,
            r,
            c,
            y_vel: 0.0,
            x_vel: 0.0,
            grabing: false,
            surface_friction,
            retention,
            stop_boucing: 0.05,
            mass,
            force: (0.0, 0.0),
        }
    }

    fn friction_checks(&mut self) {
        if self.y_vel.abs() < self.stop_boucing {
            self.y_vel = 0.0;
            self.x_vel = self.x_vel * self.surface_friction;
        }
    }

    fn euler_integration(&mut self, delta_time: f32) {
        // Newton's second law of motion: F = ma
        let x_acc = self.force.0 / self.mass;
        let y_acc = self.force.1 / self.mass;

        // Update velocity using Euler's method
        self.y_vel += y_acc * delta_time;
        self.x_vel += x_acc * delta_time;

        self.x += self.x_vel * delta_time;
        self.y += self.y_vel * delta_time;
    }

    fn apply_gravity(&mut self, delta_time: f32) {
        // Apply gravity
        let pixels_per_meter = 100.0;
        let universal_gravity_constant = 20.5; // 9.8 m/s^2
        let gravity = universal_gravity_constant * pixels_per_meter;
        self.y_vel += gravity * delta_time;
    }

    fn edges(&mut self) {
        if self.y + self.r > screen_height() {
            self.y = screen_height() - self.r;
            self.y_vel = self.y_vel * -1.0 * self.retention;
        }

        if self.x + self.r > screen_width() {
            self.x = screen_width() - self.r;
            self.x_vel = self.x_vel * -1.0 * self.retention;
        } else if self.x - self.r < 0.0 {
            self.x = self.r;
            self.x_vel = self.x_vel * -1.0 * self.retention;
        }
    }

    fn throwing_logic(&mut self, mouse_tregectory: &mut Vec<(f32, f32)>) {
        let grabing = self.is_grabing();

        if grabing == 1 {
            mouse_tregectory.push(mouse_position());
            if mouse_tregectory.len() > 20 {
                mouse_tregectory.remove(0);
            }
        } else if grabing == -1 {
            let x_push = mouse_tregectory[0].0 - mouse_tregectory[mouse_tregectory.len() - 1].0;
            let y_push = mouse_tregectory[0].1 - mouse_tregectory[mouse_tregectory.len() - 1].1;

            let x_push = x_push * -1.0;
            let y_push = y_push * -1.0;

            let x_push = x_push * 2000.0;
            let y_push = y_push * 2000.0;

            let force = (x_push, y_push);

            self.apply_force(force);
        }
    }

    fn update(&mut self, delta_time: f32) {
        self.apply_gravity(delta_time);
        self.friction_checks();
        self.euler_integration(delta_time);
        self.edges();
        if self.grabing {
            self.x = mouse_position().0;
            self.y = mouse_position().1;
            self.x_vel = 0.0;
            self.y_vel = 0.0;
        }

        // Update the force to 0
        self.force = (0.0, 0.0);

        // Draw the ball
        self.draw();
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, self.r + 2.0, WHITE);
        draw_circle(self.x, self.y, self.r, self.c);
    }

    fn is_grabing(&mut self) -> i32 {
        let mouse_pos = mouse_position();
        if is_mouse_button_pressed(MouseButton::Left)
            && mouse_pos.0 > self.x - self.r
            && mouse_pos.0 < self.x + self.r
            && mouse_pos.1 > self.y - self.r
            && mouse_pos.1 < self.y + self.r
        {
            self.grabing = true;
        } else if is_mouse_button_released(MouseButton::Left) && self.grabing {
            self.grabing = false;
            self.y_vel = 0.0;
            return -1;
        }

        if self.grabing {
            return 1;
        } else {
            return 0;
        }
    }

    fn apply_force(&mut self, force: (f32, f32)) {
        self.force = force;
    }
}
