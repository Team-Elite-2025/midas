use nalgebra::Vector2;
use std::time::{Duration, Instant};

const INTERCEPT_THRESHOLD: f64 = 1.2; // Adjust to balance between interception and safe mode
const ERROR_CORRECTION_FACTOR: f64 = 0.1; // Simple correction factor for trajectory prediction

// Function for colored logging
fn log(message: &str, color: &str) {
    let color_code = match color {
        "red" => "\x1b[31m",
        "green" => "\x1b[32m",
        "yellow" => "\x1b[33m",
        "blue" => "\x1b[34m",
        "default" => "\x1b[0m",
        _ => "\x1b[0m",
    };
    println!("{}{}\x1b[0m", color_code, message);
}

struct Ball {
    position: Vector2<f64>,
    velocity: Vector2<f64>,
    acceleration: Vector2<f64>,
    jerk: Vector2<f64>,
}

impl Ball {
    fn new(position: Vector2<f64>) -> Self {
        Self {
            position,
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            jerk: Vector2::new(0.0, 0.0),
        }
    }

    fn update(&mut self, new_position: Vector2<f64>, delta_time: f64) {
        let new_velocity = (new_position - self.position) / delta_time;
        let new_acceleration = (new_velocity - self.velocity) / delta_time;
        let new_jerk = (new_acceleration - self.acceleration) / delta_time;

        self.jerk = new_jerk;
        self.acceleration = new_acceleration;
        self.velocity = new_velocity;
        self.position = new_position;
    }

    fn predict_position(&self, delta_time: f64) -> Vector2<f64> {
        let predicted_position = self.position
            + self.velocity * delta_time
            + 0.5 * self.acceleration * delta_time.powi(2)
            + (1.0 / 6.0) * self.jerk * delta_time.powi(3);

        // Apply simple error correction
        predicted_position + ERROR_CORRECTION_FACTOR * (predicted_position - self.position)
    }
}

struct Enemy {
    position: Vector2<f64>,
    velocity: Vector2<f64>,
}

impl Enemy {
    fn new(position: Vector2<f64>, velocity: Vector2<f64>) -> Self {
        Self { position, velocity }
    }

    fn time_to_reach(&self, target: &Vector2<f64>) -> f64 {
        let distance = (target - self.position).norm();
        let speed = self.velocity.norm().max(1e-6); // Avoid division by zero
        distance / speed
    }
}

struct Goalie {
    position: Vector2<f64>,
}

impl Goalie {
    fn new(position: Vector2<f64>) -> Self {
        Self { position }
    }

    fn time_to_reach(&self, target: &Vector2<f64>) -> f64 {
        let distance = (target - self.position).norm();
        let speed = 2.0; // Assume a fixed speed for the goalie
        distance / speed
    }

    fn intercept(&mut self, ball: &Ball, rendezvous: Vector2<f64>) {
        log(&format!("[ACTION] Intercepting the ball at {:?}!", rendezvous), "red");
        self.position = rendezvous;
    }

    fn safe_mode(&mut self, ball: &Ball) {
        log("[INFO] Entering safe mode.", "green");
        self.position = Vector2::new(0.0, 0.0);
    }

    fn decide_action(
        &mut self,
        ball: &Ball,
        enemy: &Enemy,
        delta_time: f64,
    ) {
        let predicted_ball_position = ball.predict_position(delta_time);
        log(&format!("[TRAJECTORY] Predicted ball position: {:?}", predicted_ball_position), "blue");
        log(&format!("[INFO] Current ball position: {:?}", ball.position), "green");

        let goalie_time = self.time_to_reach(&predicted_ball_position);
        let enemy_time = enemy.time_to_reach(&predicted_ball_position);

        log(&format!("[TIME] Goalie time to intercept: {:.2}s", goalie_time), "yellow");
        log(&format!("[TIME] Enemy time to intercept: {:.2}s", enemy_time), "yellow");

        if goalie_time <= enemy_time * INTERCEPT_THRESHOLD {
            log("[INFO] Goalie can intercept the ball before the enemy.", "yellow");
            self.intercept(ball, predicted_ball_position);
        } else {
            log("[INFO] Enemy might reach the ball first. Entering safe mode.", "green");
            self.safe_mode(ball);
        }
    }

    fn in_target_box(ball: &Ball) -> bool {
        (ball.position.x.abs() <= 10.0) && (ball.position.y.abs() <= 20.0)
    }
}

fn main() {
    let mut goalie = Goalie::new(Vector2::new(0.0, 0.0));
    let mut ball = Ball::new(Vector2::new(0.0, 6.0));
    let enemy = Enemy::new(Vector2::new(1.0, 5.0), Vector2::new(0.2, -0.1));

    let mut last_update = Instant::now();

    loop {
        let now = Instant::now();
        let delta_time = now.duration_since(last_update).as_secs_f64();
        last_update = now;

        ball.update(ball.position + Vector2::new(0.1, -0.2), delta_time);

        if Goalie::in_target_box(&ball) {
            goalie.decide_action(&ball, &enemy, delta_time);
        } else {
            log("[INFO] Ball is outside the target box. Maintaining position.", "default");
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}

