use nalgebra::Vector2;

pub const DIAMETER: f32 = 90.0;
const DEPTH: i32 = 0;
pub enum LoopState {
    Ready,
    EnemiesSelection,
    InitialSelection,
}

pub struct Environment {
    pub enemies: [Vector2<f32>; 3],
    pub field_max: Vector2<f32>,
    // pub ball: Vector2<f32>,
}
pub struct Setup {
    pub enemies: Vec<Vector2<f32>>,
    pub paths: Vec<Vec<Vector2<f32>>>,
    pub key_points: Vec<Vector2<f32>>,
    pub env: Option<Environment>,
    pub state: LoopState,
}

impl Setup {
    pub fn build_min_path(&mut self) {
        let right = build_path_plan(self.env.as_ref().unwrap(), &self.key_points, DEPTH, -1.0);

        let left = build_path_plan(self.env.as_ref().unwrap(), &self.key_points, DEPTH, 1.0);

        let left_len = path_length(&left);
        let right_len = path_length(&right);

        if left_len < right_len {
            self.paths = vec![left, right];
        } else if left_len > right_len {
            self.paths = vec![right, left];
        } else {
            self.paths = vec![left];
        }
    }
}

fn build_path_plan(
    env: &Environment,
    trajectory: &[Vector2<f32>],
    depth: i32,
    direction: f32,
) -> Vec<Vector2<f32>> {
    if depth >= 2 {
        return trajectory.to_vec();
    }

    if let Some(obstacle_point) = is_obstacle(env, &trajectory) {
        let start = trajectory.first().unwrap();
        let goal = trajectory.last().unwrap();

        let mut obstacle = obstacle_point;
        let mut subgoal;

        loop {
            subgoal = search_point(*start, obstacle, direction);

            if !is_point_obstacle(env, subgoal) {
                break;
            }

            obstacle = subgoal;
        }

        let path1 = build_path_plan(
            env,
            &generate_straight(*start, subgoal),
            depth + 1,
            direction,
        );
        let path2 = build_path_plan(
            env,
            &generate_straight(subgoal, *goal),
            depth + 1,
            direction,
        );

        return join_straight(path1, path2);
    }

    trajectory.to_vec()
}

fn search_point(from: Vector2<f32>, obstacle: Vector2<f32>, sign: f32) -> Vector2<f32> {
    let direction = obstacle - from;

    if direction.norm() == 0.0 {
        return obstacle;
    }

    let perp = Vector2::new(-sign * direction.y, sign * direction.x).normalize();

    obstacle + perp * (DIAMETER + 15.0)
}

fn is_point_obstacle(env: &Environment, point: Vector2<f32>) -> bool {
    //I need to guarantee that the search_point will choose a point inside the field limits
    // let min_bounds = Vector2::new(0.0, 0.0);

    // if point.x < min_bounds.x
    //     || point.y < min_bounds.y
    //     || point.x > env.field_max.x
    //     || point.y > env.field_max.y
    // {
    //     return true;
    // }

    for enemy in &env.enemies {
        let distance = (point - enemy).norm();
        if distance < DIAMETER / 2.0 {
            return true;
        }
    }

    false
}

fn is_obstacle(env: &Environment, trajectory: &[Vector2<f32>]) -> Option<Vector2<f32>> {
    if trajectory.len() < 2 {
        return None;
    }

    for window in trajectory.windows(2) {
        let a = window[0];
        let b = window[1];

        let steps = 150;
        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let point = a + (b - a) * t;

            for enemy in &env.enemies {
                let distance = (point - enemy).norm();
                if distance < DIAMETER / 2.0 {
                    return Some(*enemy);
                }
            }
        }
    }

    None
}

fn generate_straight(p1: Vector2<f32>, p2: Vector2<f32>) -> Vec<Vector2<f32>> {
    vec![p1, p2]
}

fn join_straight(t1: Vec<Vector2<f32>>, t2: Vec<Vector2<f32>>) -> Vec<Vector2<f32>> {
    let mut result = t1;
    result.extend_from_slice(&t2[1..]);
    result
}

fn path_length(path: &[Vector2<f32>]) -> f32 {
    let mut length = 0.0;

    for window in path.windows(2) {
        length += (window[1] - window[0]).norm();
    }

    length
}
