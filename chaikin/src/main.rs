use macroquad::prelude::*;

#[macroquad::main("CHAIKING")]
async fn main() {
    let mut points: Vec<Vec2> = Vec::new();
    let mut points_fix: Vec<Vec2> = Vec::new();
    let mut stop_points = true;
    let mut start = false;
    let mut smoothing = false;
    let mut frame_counter = 0;
    let smoothing_interval = 30; 
    let mut line_one = false ;
    let mut step_count = 0;
    let max_steps = 7;

    loop {
        clear_background(BLACK);
        if !line_one && stop_points && is_mouse_button_pressed(MouseButton::Left) {
            points_fix.push(mouse_position().into());
            points.push(mouse_position().into());
        }


        if start || line_one{
            for i in 0..points.len().saturating_sub(1) {
                let p0 = points[i];
                let p1 = points[i + 1];
                draw_line(p0.x, p0.y, p1.x, p1.y, 2.0, RED);
            }
        }


        for p in &points_fix {
            draw_circle(p.x, p.y, 4.0, GRAY);
            draw_circle(p.x, p.y, 2.0, BLACK);
        }

        if is_key_pressed(KeyCode::Enter) && points.len() >= 3 {
            start = true;
            stop_points = false;
            smoothing = true;
            frame_counter = 0;
            step_count = 0; 
        }
        if is_key_pressed(KeyCode::Enter) && points.len() == 2{
            line_one = true ;
        }

        if is_key_pressed(KeyCode::C) {
            points.clear();
            points_fix.clear();
            stop_points = true;
            start = false;
            smoothing = false;
            frame_counter = 0;
            step_count = 0; 
            line_one = false ;
        }

        if smoothing && step_count < max_steps {
            frame_counter += 1;

            if frame_counter >= smoothing_interval {
                frame_counter = 0;

                let mut new_points = Vec::new();
                new_points.push(points[0]);

                for i in 0..points.len() - 1 {
                    let p0 = points[i];
                    let p1 = points[i + 1];
                    let q = p0 * 0.75 + p1 * 0.25;
                    let r = p0 * 0.25 + p1 * 0.75;
                    new_points.push(q);
                    new_points.push(r);
                }

                new_points.push(points[points.len() - 1]);
                points = new_points;

                step_count += 1;

           
            }
        }
        if step_count == 7{
            points = points_fix.clone();
            step_count= 0;
        }
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // UI Text
        draw_text("Click to add points. Press Enter to start smoothing.", 20.0, 20.0, 20.0, WHITE);
        draw_text("Press Escape to quit.", 20.0, 50.0, 20.0, WHITE);
        draw_text("Press C to restart the drawing.", 20.0, 80.0, 20.0, WHITE);
        draw_text(&format!("Steps: {}/{}", step_count, max_steps), 20.0, 110.0, 20.0, WHITE);
        next_frame().await;
    }

    println!("Final Points: {:?}", points);
}

