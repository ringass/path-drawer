use nalgebra::Vector2;

pub const DIAMETER: f32 = 90.0;

pub enum LoopState {
    Ready,
    EnemiesSelection,
    InitialSelection,
}

pub struct Environment {
    pub enemies: [Vector2<f32>; 3],
    // pub field_max: Vector2<f32>,
    // pub ball: Vector2<f32>,
}
pub struct Setup {
    pub enemies: Vec<Vector2<f32>>,
    pub path: Vec<Vector2<f32>>,
    pub key_points: Vec<Vector2<f32>>,
    pub env: Option<Environment>,
    pub state: LoopState,
}

pub fn build_path_plan(
    env: &Environment,
    trajectory: Vec<Vector2<f32>>,
    depth: i32,
) -> Vec<Vector2<f32>> {
    if depth >= 6 {
        return trajectory;
    }

    if let Some(obstacle_point) = is_obstacle(env, &trajectory) {
        let start = trajectory.first().unwrap();
        let goal = trajectory.last().unwrap();

        let mut subgoal = search_point(*start, obstacle_point, -1.0);

        while is_point_obstacle(env, subgoal) {
            subgoal = search_point(subgoal, subgoal, 1.0);
        }

        let path1 = build_path_plan(env, generate_straight(*start, subgoal), depth + 1);
        let path2 = build_path_plan(env, generate_straight(subgoal, *goal), depth + 1);

        return join_straight(path1, path2);
    }

    trajectory
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
    for enemy in &env.enemies {
        let distance = (point - enemy).norm();
        if distance < DIAMETER / 2.0 {
            return true;
        }
    }
    false
}

fn is_obstacle(env: &Environment, trajectory: &Vec<Vector2<f32>>) -> Option<Vector2<f32>> {
    if trajectory.len() < 2 {
        return None;
    }

    for window in trajectory.windows(2) {
        let a = window[0];
        let b = window[1];

        let steps = 100;
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
