use macroquad::prelude::*;
use nalgebra::Vector2;
use path::*;

const DEPTH: i32 = 0;
#[macroquad::main("STO'x pathplanning")]
async fn main() {
    let mut traject_flag = false;
    let mut enemies_flag = false;
    let mut trajectory_points: Vec<Vector2<f32>> = vec![];
    let mut enemies: Vec<Vector2<f32>> = vec![];
    let mut path: Vec<Vector2<f32>> = vec![];
    let mut env: Option<Environment> = None;

    loop {
        clear_background(Color::new(22.0 / 255.0, 24.0 / 255.0, 24.0 / 255.0, 0.91));

        //left mouse to define start and goal points
        if is_mouse_button_pressed(MouseButton::Left) && !traject_flag {
            trajectory_points.push(Vector2::new(mouse_position().0, mouse_position().1));

            if trajectory_points.len() == 2 {
                traject_flag = true;
            }
        }

        //left mouse to define the 3 enemy robots
        if is_mouse_button_pressed(MouseButton::Right) && !enemies_flag {
            enemies.push(Vector2::new(mouse_position().0, mouse_position().1));

            if enemies.len() == 3 {
                enemies_flag = true;
            }
        }

        //start path planning
        if traject_flag && enemies_flag && env.is_none() {
            env = Some(Environment {
                enemies: [enemies[0], enemies[1], enemies[2]],
            });

            path = build_path_plan(env.as_ref().unwrap(), trajectory_points.clone(), DEPTH);
        }

        //reset window
        if is_key_pressed(KeyCode::R) {
            trajectory_points.clear();
            enemies.clear();
            path.clear();
            env = None;
            traject_flag = false;
            enemies_flag = false;
        }

        draw_path(path.clone(), enemies.clone(), trajectory_points.clone());
        draw_text_center(enemies_flag, traject_flag);

        next_frame().await;
    }
}

fn draw_path(
    path: Vec<Vector2<f32>>,
    enemies: Vec<Vector2<f32>>,
    trajectory_points: Vec<Vector2<f32>>,
) {
    for window in path.windows(2) {
        let a = window[0];
        let b = window[1];

        draw_line(a.x, a.y, b.x, b.y, 2.0, WHITE);
    }

    for point in &path {
        draw_circle(point.x as f32, point.y as f32, 5.0, RED);
    }

    for enemy in &enemies {
        draw_circle(enemy.x as f32, enemy.y as f32, 45.0, BLUE);
    }

    for point in &trajectory_points {
        draw_circle(point.x, point.y, 6.0, GREEN);
    }
}

fn draw_text_center(enemies_flag: bool, traject_flag: bool) {
    let text = if !traject_flag {
        "MOUSELEFT to define start and goal"
    } else if !enemies_flag {
        "MOUSERIGHT to define three enemies"
    } else {
        "R to reset"
    };

    let font_size = 30.0;
    let dimensions = measure_text(text, None, font_size as u16, 1.0);
    let x = screen_width() / 2.0 - dimensions.width / 2.0;
    let y = 30.0;

    draw_text(text, x, y, font_size, YELLOW);
}
