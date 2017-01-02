extern crate orbclient;
extern crate tachyonic;
extern crate nalgebra as na;


use tachyonic::{ RenderContext };
use na::{ Vector3, Matrix4, Perspective3, PerspectiveMatrix3, Inverse };
use std::time::Instant;


fn get_perspective_matrix(fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> PerspectiveMatrix3<f32> {

    let matrix = Perspective3::new(aspect_ratio, fov, z_near, z_far).to_perspective_matrix().as_matrix().inverse().unwrap();
    PerspectiveMatrix3::from_matrix_unchecked(matrix)
}

fn main() {

    let mut start = Instant::now();
    let ww = 800;
    let wh = 600;
    let mut render_context = RenderContext::new(ww, wh, "tachyonic");

    let aspect_ratio = ww as f32 / wh as f32;
    let fov = 100_f32;
    let z_near = 0.1_f32;
    let z_far = 1000_f32;

    let p_matrix = get_perspective_matrix(fov, aspect_ratio, z_near, z_far);


//    let top = Vector3::new(0.5, 0.01, 1.0);
//    let left = Vector3::new(0.01, 0.75, 1.0);
//    let right = Vector3::new(0.99, 0.99, 1.0);

//    let top = Vector3::new(0.1, 0.01, 1.0);
//    let left = Vector3::new(0.01, 0.1, 1.0);
//    let right = Vector3::new(0.15, 0.15, 1.0);

    let top = Vector3::new(0.5, 0.0, 10.0);
    let left = Vector3::new(0.0, 1.0, 1.0);
    let right = Vector3::new(1.0, 1.0, 1.0);

//    println!("top: {:?}", right);
//
//    let top = p_matrix.project_vector(&top);
//    let left = p_matrix.project_vector(&left);
//    let right = p_matrix.project_vector(&right);
//
//    println!("top: {:?}", right);

    let mut frame_cnt = 0_f32;
    let mut counter_duration = 0_f32;

    let ww_f = ww as f32;
    let wh_f  = wh as f32;

    'events: loop {

        let end = Instant::now();
        let delta = end.duration_since(start);
        let delta_ms = delta.as_secs() as f32 * 1000_f32 + (delta.subsec_nanos() as f32)/1000000 as f32;
        start = Instant::now();
//         println!("{:?} ms", delta_ms);

        //to raster space
//        let rtop = Vector3::new(ww_f * top.x , wh_f * top.y, 1_f32);
//        let rleft = Vector3::new(ww_f * left.x, wh_f * left.y, 1_f32);
//        let rright = Vector3::new(ww_f * right.x , wh_f * right.y, 1_f32);



        render_context.draw_triangle1(top, left, right);
        render_context.sync();

        frame_cnt += 1_f32;
        counter_duration += delta_ms;
        if counter_duration > 1000_f32 {
            println!("FPS: {}", frame_cnt / counter_duration * 1000_f32);
            frame_cnt = 0_f32;
            counter_duration = 0_f32;
        }


        for orbital_event in render_context.events() {
            use orbclient::event::EventOption;

            match orbital_event.to_option() {
                EventOption::Key(key_event) => {
                    match key_event.scancode {
                        //Translation
                        orbclient::K_W => println!("w key"),
                        _ => println!("other key"),
                    }
                }
                EventOption::Quit(..) => break 'events,
//                event_option => println!("{:?}", event_option)
                _ => {}
            }
        }
    }
}