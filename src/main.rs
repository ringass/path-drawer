use macroquad::prelude::*;
use nalgebra::Vector2;
use path::*;

const COLOR: Color = Color::new(22.0 / 255.0, 24.0 / 255.0, 24.0 / 255.0, 0.91);

#[macroquad::main("STO'x pathplanning")]
async fn main() {
    let mut setup = Setup {
        enemies: vec![],
        env: None,
        paths: vec![vec![], vec![]],
        key_points: vec![],
        state: LoopState::InitialSelection,
    };

    loop {
        clear_background(COLOR);

        match setup.state {
            LoopState::InitialSelection => {
                //left mouse to define start and goal points
                if is_mouse_button_pressed(MouseButton::Left) && free_to_draw(&setup) {
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
                if is_mouse_button_pressed(MouseButton::Right) && free_to_draw(&setup) {
                    setup
                        .enemies
                        .push(Vector2::new(mouse_position().0, mouse_position().1));

                    if setup.enemies.len() == 3 && setup.env.is_none() {
                        setup.env = Some(Environment {
                            enemies: [setup.enemies[0], setup.enemies[1], setup.enemies[2]],
                            field_max: Vector2::new(screen_width(), screen_height()),
                        });

                        setup.build_min_path();

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
            setup.paths = vec![vec![], vec![]];
            setup.env = None;
            setup.state = LoopState::InitialSelection;
        }

        draw_path(&setup);
        draw_text_center(&setup.state);

        next_frame().await;
    }
}

fn draw_path(setup: &Setup) {
    for (i, path) in setup.paths.iter().enumerate() {
        for window in path.windows(2) {
            let a = window[0];
            let b = window[1];

            draw_line(a.x, a.y, b.x, b.y, 2.0, if i == 0 { WHITE } else { RED });
        }

        for point in path {
            draw_circle(point.x as f32, point.y as f32, 5.0, YELLOW);
        }
    }

    for enemy in &setup.enemies {
        draw_circle(enemy.x as f32, enemy.y as f32, 45.0, BLUE);
    }

    match (setup.key_points.first(), setup.key_points.last()) {
        (Some(first), Some(last)) => {
            draw_circle(first.x, first.y, 6.0, GREEN);
            draw_circle(last.x, last.y, 6.0, DARKGREEN);
        }
        _ => {}
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

fn free_to_draw(setup: &Setup) -> bool {
    let mouse_v2 = Vector2::new(mouse_position().0, mouse_position().1);

    for enemy in &setup.enemies {
        if (mouse_v2 - enemy).norm() < DIAMETER {
            return false;
        }
    }

    for point in &setup.key_points {
        if (mouse_v2 - point).norm() < 6.0 {
            return false;
        }
    }

    true
}
