use macroquad::prelude::*;
use nalgebra::Vector2;
use path::*;

const DEPTH: i32 = 0;
#[macroquad::main("STO'x pathplanning")]
async fn main() {
    let mut setup = Setup {
        enemies: vec![],
        env: None,
        path: vec![],
        key_points: vec![],
        state: LoopState::InitialSelection,
    };

    loop {
        clear_background(Color::new(22.0 / 255.0, 24.0 / 255.0, 24.0 / 255.0, 0.91));

        match setup.state {
            LoopState::InitialSelection => {
                //left mouse to define start and goal points
                if is_mouse_button_pressed(MouseButton::Left) {
                    setup
                        .key_points
                        .push(Vector2::new(mouse_position().0, mouse_position().1));

                    if setup.key_points.len() == 2 {
                        setup.state = LoopState::EnemiesSelection;
                    }
                }
            }
            LoopState::EnemiesSelection => {
                //left mouse to define the 3 enemy robots
                if is_mouse_button_pressed(MouseButton::Right) {
                    setup
                        .enemies
                        .push(Vector2::new(mouse_position().0, mouse_position().1));

                    if setup.enemies.len() == 3 && setup.env.is_none() {
                        setup.env = Some(Environment {
                            enemies: [setup.enemies[0], setup.enemies[1], setup.enemies[2]],
                        });

                        setup.path = build_path_plan(
                            setup.env.as_ref().unwrap(),
                            setup.key_points.clone(),
                            DEPTH,
                        );

                        setup.state = LoopState::Ready;
                    }
                }
            }
            LoopState::Ready => {}
        }

        //reset window
        if is_key_pressed(KeyCode::R) {
            setup.key_points.clear();
            setup.enemies.clear();
            setup.path.clear();
            setup.env = None;
            setup.state = LoopState::InitialSelection;
        }

        draw_path(&setup.path, &setup.enemies, &setup.key_points);
        draw_text_center(&setup.state);

        next_frame().await;
    }
}

fn draw_path(
    path: &Vec<Vector2<f32>>,
    enemies: &Vec<Vector2<f32>>,
    key_points: &Vec<Vector2<f32>>,
) {
    for window in path.windows(2) {
        let a = window[0];
        let b = window[1];

        draw_line(a.x, a.y, b.x, b.y, 2.0, WHITE);
    }

    for point in path {
        draw_circle(point.x as f32, point.y as f32, 5.0, RED);
    }

    for enemy in enemies {
        draw_circle(enemy.x as f32, enemy.y as f32, 45.0, BLUE);
    }

    for point in key_points {
        draw_circle(point.x, point.y, 6.0, GREEN);
    }
}

fn draw_text_center(state: &LoopState) {
    let text = match state {
        LoopState::InitialSelection => "MOUSELEFT to define start and goal",
        LoopState::EnemiesSelection => "MOUSERIGHT to define three enemies",
        LoopState::Ready => "R to reset",
    };

    let font_size = 30.0;
    let dimensions = measure_text(text, None, font_size as u16, 1.0);
    let x = screen_width() / 2.0 - dimensions.width / 2.0;
    let y = 30.0;

    draw_text(text, x, y, font_size, YELLOW);
}
