use nalgebra::Vector2;

/// Represents a robot's trajectory using a Cubic Bézier curve
#[derive(Debug, Clone)]
pub struct RobotTrajectory {
    start_pos: Vector2<f64>,
    ball_pos: Vector2<f64>,
    goal_pos: Vector2<f64>,
    control_points: [Vector2<f64>; 4],
}

/// Contains key trajectory information for quick access
#[derive(Debug, Clone)]
pub struct TrajectoryInfo {
    pub start_pos: (f64, f64),
    pub ball_pos: (f64, f64),
    pub goal_pos: (f64, f64),
    pub control_points: [(f64, f64); 4],
}

impl RobotTrajectory {
    /// Create a new robot trajectory
    pub fn new(start_pos: (f64, f64), ball_pos: (f64, f64), goal_pos: (f64, f64)) -> Self {
        let start = Vector2::new(start_pos.0, start_pos.1);
        let ball = Vector2::new(ball_pos.0, ball_pos.1);
        let goal = Vector2::new(goal_pos.0, goal_pos.1);

        let control_points = Self::generate_bezier_control_points(&start, &ball);

        Self {
            start_pos: start,
            ball_pos: ball,
            goal_pos: goal,
            control_points,
        }
    }

    /// Generate Bézier curve control points
    fn generate_bezier_control_points(start: &Vector2<f64>, end: &Vector2<f64>) -> [Vector2<f64>; 4] {
        let x0 = start.x;
        let y0 = start.y;
        let x3 = end.x;
        let y3 = end.y;

        let x1 = x0 + (x3 - x0) / 3.0;
        let x2 = x3 - (x3 - x0) / 3.0;
        let y1 = y0 + (y3 - y0) / 3.0;
        let y2 = y3 - (y3 - y0) / 3.0;

        [
            Vector2::new(x0, y0),
            Vector2::new(x1, y1),
            Vector2::new(x2, y2),
            Vector2::new(x3, y3)
        ]
    }

    /// Compute position at parameter t
    pub fn compute_position(&self, t: f64) -> Vector2<f64> {
        let one_minus_t = 1.0 - t;
        
        one_minus_t.powi(3) * self.control_points[0] +
        3.0 * one_minus_t.powi(2) * t * self.control_points[1] +
        3.0 * one_minus_t * t.powi(2) * self.control_points[2] +
        t.powi(3) * self.control_points[3]
    }

    /// Compute movement vector for given delta t
    pub fn compute_delta_vector(&self, delta_t: f64) -> Vector2<f64> {
        self.compute_position(delta_t) - self.compute_position(0.0)
    }

    /// Utility function to get trajectory information
    pub fn get_trajectory_info(&self) -> TrajectoryInfo {
        TrajectoryInfo {
            start_pos: (self.start_pos.x, self.start_pos.y),
            ball_pos: (self.ball_pos.x, self.ball_pos.y),
            goal_pos: (self.goal_pos.x, self.goal_pos.y),
            control_points: [
                (self.control_points[0].x, self.control_points[0].y),
                (self.control_points[1].x, self.control_points[1].y),
                (self.control_points[2].x, self.control_points[2].y),
                (self.control_points[3].x, self.control_points[3].y),
            ],
        }
    }

    // Debug/development methods (optional)
    pub fn print_path_details(&self) {
        println!("Trajectory Details:");
        println!("Start Position: ({}, {})", self.start_pos.x, self.start_pos.y);
        println!("Ball Position: ({}, {})", self.ball_pos.x, self.ball_pos.y);
        println!("Goal Position: ({}, {})", self.goal_pos.x, self.goal_pos.y);

        println!("\nControl Points:");
        let point_labels = ["Start", "Control Point 1", "Control Point 2", "End"];
        for (label, point) in point_labels.iter().zip(self.control_points.iter()) {
            println!("{}: ({}, {})", label, point.x, point.y);
        }
    }
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trajectory() {
        // Robot starts at (0,0), hits ball at (10,5), aiming for goal at (15,8)
        let trajectory = RobotTrajectory::new(
            (0.0, 0.0),
            (10.0, 5.0),
            (15.0, 8.0)
        );

        // Print trajectory details
        trajectory.print_path_details();

        // Sample points along the trajectory
        let sample_points = [0.0, 0.25, 0.5, 0.75, 1.0];
        println!("\nSample Points on Trajectory:");
        for t in sample_points.iter() {
            let pos = trajectory.compute_position(*t);
            println!("Position at t={:.2}: ({}, {})", t, pos.x, pos.y);
        }

        // Compute movement vector
        let delta_vector = trajectory.compute_delta_vector(0.5);
        println!("\nMovement Vector (delta_t = 0.5): ({}, {})", delta_vector.x, delta_vector.y);

        // Get trajectory info
        let info = trajectory.get_trajectory_info();
        println!("\nTrajectory Info: {:?}", info);
    }
}
