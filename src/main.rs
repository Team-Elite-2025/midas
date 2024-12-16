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

    fn bezier_path_blocked(&self, target: &Vector2<f64>, ball: &Ball, enemy: &Enemy) -> bool {
        // Simulate a simple bezier curve collision check
        let mid_point = (self.position + target) * 0.5;
        let path_points = vec![self.position, mid_point, *target];

        for t in (0..=10).map(|i| i as f64 / 10.0) {
            let bezier_point = (1.0 - t).powi(2) * path_points[0]
                + 2.0 * (1.0 - t) * t * path_points[1]
                + t.powi(2) * path_points[2];

            if (bezier_point - enemy.position).norm() < 1.0 {
                return true; // Path intersects enemy
            }
        }

        false
    }

    fn intercept(&mut self, ball: &Ball, target: Vector2<f64>, delta_time: f64) {
        log(
            &format!(
                "[ACTION] Intercepting at position {:?}, moving towards {:?}",
                ball.position, target
            ),
            "red",
        );

        let movement_vector = (target - self.position) / delta_time;
        log(&format!("[INFO] Movement vector: {:?}", movement_vector), "blue");

        self.position = target; // Update position to the interception target
    }

    fn safe_mode(&mut self) {
        log("[INFO] Entering safe mode.", "green");
        self.position = Vector2::new(0.0, 0.0); // Reset position
    }

    fn decide_action(
        &mut self,
        ball: &Ball,
        enemy: &Enemy,
        teammate_position: Vector2<f64>,
        goal_position: Vector2<f64>,
        delta_time: f64,
    ) {
        let predicted_ball_position = ball.predict_position(delta_time);
        log(
            &format!(
                "[DEBUG] Predicted ball position in {}s: {:?}",
                delta_time, predicted_ball_position
            ),
            "blue",
        );

        let goalie_time = self.time_to_reach(&predicted_ball_position);
        let enemy_time = enemy.time_to_reach(&predicted_ball_position);

        if goalie_time <= enemy_time * INTERCEPT_THRESHOLD {
            let target = if self.bezier_path_blocked(&goal_position, ball, enemy) {
                log("[INFO] Path to goal is blocked. Passing to teammate.", "yellow");
                teammate_position
            } else {
                log("[INFO] Path to goal is clear. Shooting towards goal.", "yellow");
                goal_position
            };

            self.intercept(ball, target, delta_time);
        } else {
            log("[INFO] Enemy might reach the ball first. Entering safe mode.", "green");
            self.safe_mode();
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
    let teammate_position = Vector2::new(-5.0, 10.0);
    let goal_position = Vector2::new(0.0, -20.0);

    let mut last_update = Instant::now();

    loop {
        let now = Instant::now();
        let delta_time = now.duration_since(last_update).as_secs_f64();
        last_update = now;

        // Simulate ball movement update (replace with actual sensor data in real implementation)
        ball.update(ball.position + Vector2::new(0.1, -0.2), delta_time);

        if Goalie::in_target_box(&ball) {
            goalie.decide_action(&ball, &enemy, teammate_position, goal_position, delta_time);
        } else {
            log("[INFO] Ball is outside the target box. Maintaining position.", "default");
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}

