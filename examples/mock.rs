#![feature(step_by)]
extern crate orbclient;
extern crate tachyonic;
extern crate nalgebra as na;

use orbclient::Color;
use tachyonic::{ RenderContext, Mesh, Texture, Vertex };
use na::{ Vector3, Point3, Point4, Point2, Matrix4, Perspective3, PerspectiveMatrix3, Inverse, Eye, Isometry3, Rotation3, Origin, ToHomogeneous };
use std::time::Instant;

//x0-x1 input range y0-y1 output range : x input cursor
pub fn lerp(x0: f32, x1: f32, y0: f32, y1: f32, x: f32) -> f32 {
    y0 + (x - x0) * ((y1 - y0) / (x1 - x0))
}

pub struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
}

impl Camera {
    pub fn new(position: Point3<f32>, target: Point3<f32>, up: Vector3<f32>) -> Self {
        Camera {
            position: position,
            target: target,
            up: up,
        }
    }
}

////AOS <-> SOA ?
//pub struct Vertex {
//    pub position: Point4<f32>,
//    pub texture_coordinate: Point2<f32>,
//}
//
//impl Vertex {
//    pub fn new(position: Point4<f32>, texture_coordinate: Point2<f32>, color: Color) -> Self {
//        Vertex {
//            position: position,
//            texture_coordinate: texture_coordinate,
//        }
//    }
//}
//
//pub struct Mesh {
//    pub vertices: Vec<Point4<f32>>,
//    pub tvertices: Vec<Vertex>,
//    pub position: Point3<f32>,
//    pub rotation: Vector3<f32>,
//}
//
//impl Mesh {
//    pub fn new() -> Self {
//        Mesh {
//            vertices: Vec::new(),
//            tvertices: Vec::new(),
//            position: Point3::new(0_f32, 0_f32, 0_f32),
//            rotation: Vector3::new(0_f32, 0_f32, 0_f32),
//        }
//    }
//}
//
//pub struct Texture {
//
//}
//
//impl Texture {
//    pub fn get(&self, u: f32, v: f32) -> Color {
//
//        if(u == 0.5 && v == 0.0) {
//            Color::rgba(255, 0, 0, 255)
//        } else if (u == 0.0 && v == 1.0) {
//            Color::rgba(0, 255, 0, 255)
//        } else {
//            Color::rgba(0, 0, 255, 255)
//        }
//
//    }
//}

fn project(point: Point4<f32>, mat: Matrix4<f32>, ww: u32, wh: u32) -> Point2<f32> {

    //transform into screenspace
    let mut screen_space_point = mat * point;

    //perspective divide
    screen_space_point.x = screen_space_point.x / screen_space_point.w;
    screen_space_point.y = screen_space_point.y / screen_space_point.w;

    //transform into rasterspace
//    let rx = lerp(-1.0, 1.0, 0.0, ww as f32, screen_space_point.x);
//    let ry = lerp(1.0, -1.0, 0.0, wh as f32, screen_space_point.y); //flip y
    let mut rx = (screen_space_point.x + 1.0) / 2.0 * ww as f32;
    let mut ry = (1.0 - screen_space_point.y) / 2.0 * wh as f32;  //flip y

    Point2::new(rx, ry)
}

fn main() {

    let mut start = Instant::now();
    let ww = 800;
    let wh = 600;
    let mut render_context = RenderContext::new(ww, wh, "tachyonic");

    let mut fov = 100_f32;//1.50_f32; //radians
    let aspr = ww as f32 / wh as f32;
    let z_near = 0.1_f32;
    let z_far = 1000_f32;

    //prepare data in object space in homogeneous notation
//    let mut cube = Mesh::new();
//    cube.vertices.push(Point4::new(-1_f32, 1_f32, 1_f32, 1_f32));
//    cube.vertices.push(Point4::new(1_f32, 1_f32, 1_f32, 1_f32));
//    cube.vertices.push(Point4::new(-1_f32, -1_f32, 1_f32, 1_f32));
//    cube.vertices.push(Point4::new(-1_f32, -1_f32, -1_f32, 1_f32));
//    cube.vertices.push(Point4::new(-1_f32, 1_f32, -1_f32, 1_f32));
//    cube.vertices.push(Point4::new(1_f32, 1_f32, -1_f32, 1_f32));
//    cube.vertices.push(Point4::new(1_f32, -1_f32, 1_f32, 1_f32));
//    cube.vertices.push(Point4::new(1_f32, -1_f32, -1_f32, 1_f32));

    let mut triangle = Mesh::new();
    triangle.vertices.push(Point4::new(0.0, -0.5, 0.0, 1.0)); // top
    triangle.vertices.push(Point4::new(-0.5, 0.5, 0.0, 1.0)); // left
    triangle.vertices.push(Point4::new(0.5, 0.5, 0.0, 1.0)); // right

    triangle.tvertices.push(Vertex::new(Point4::new(0.0, -0.5, 0.0, 1.0), Point2::new(0.5, 0.0), Color::rgba(255, 0, 0, 255))); // top
    triangle.tvertices.push(Vertex::new(Point4::new(-0.5, 0.5, 0.0, 1.0), Point2::new(0.0, 1.0), Color::rgba(0, 255, 0, 255))); // left
    triangle.tvertices.push(Vertex::new(Point4::new(0.5, 0.5, 0.0, 1.0), Point2::new(1.0, 1.0), Color::rgba(0, 0, 255, 255))); // right

    let tex = Texture::new();

    let mut meshes: Vec<Mesh> = Vec::new();
//    meshes.push(cube);
    meshes.push(triangle);

    let mut camera = Camera::new(Point3::new(0.0, 0.0, 1.0), Point3::origin(), Vector3::y());

    let mut frame_cnt = 0_f32;
    let mut counter_duration = 0_f32;

    'events: loop {

        let end = Instant::now();
        let delta = end.duration_since(start);
        let delta_ms = delta.as_secs() as f32 * 1000_f32 + (delta.subsec_nanos() as f32)/1000000 as f32;
        start = Instant::now();
        //         println!("{:?} ms", delta_ms);

        //update
        {

            meshes[0].rotation.y += 1_f32 as f32 / 1000_f32;
//            triangle.rotation.y += 1_f32 as f32 / 1000_f32;
//            cube.rotation.x += 1_f32 as f32 / 1000_f32;

//            camera.target.x += 1_f32 as f32 / 1000_f32;
//            camera.position.y -= 1_f32 as f32 / 1000_f32;

//            fov += 1_f32 / 100_f32;

        }



        //clear
        {
            render_context.clear();
        }

        //draw
        {

            //make this a function of the camera .. get view matrix
            //project the object from world space into camera space
            let view_matrix = Isometry3::look_at_lh(&camera.position , &camera.target, &camera.up).to_homogeneous();

            //make this properties of the camera
            //project the object from camera space into normalized screen space [-1,1]
            // needs to be transformed into raster space [0, screen_width] see fn project()
            let pm = PerspectiveMatrix3::new(aspr, fov.to_radians(), z_near, z_far);
            let projection_matrix = pm.as_matrix().clone();

//            //draw every mesh
            for mesh in &meshes {

                //make this a function of the mesh/object
                //project the object from object space into world space
                let world_matrix = Isometry3::new((*mesh).position.to_vector(), (*mesh).rotation).to_homogeneous();

                //combine all three in the correct(opposite) order and project from
                //object_space -> world_space -> camera_space -> screen_space
                let transformation_matrix = projection_matrix * view_matrix * world_matrix;

                //draw the vertices/faces/triangles of the mesh
//                for vertex in &(*mesh).vertices {
                for idx in (0..mesh.vertices.len()).step_by(3) {

                    //apply transform to the vertex and and map to raster space
                    let vert1 = project(mesh.vertices[idx as usize], transformation_matrix, ww, wh);
                    let vert2 = project(mesh.vertices[(idx + 1) as usize], transformation_matrix, ww, wh);
                    let vert3 = project(mesh.vertices[(idx + 2) as usize], transformation_matrix, ww, wh);

                    let vert_t_1 = &mesh.tvertices[idx as usize];
                    let vert_t_2 = &mesh.tvertices[(idx + 1) as usize];
                    let vert_t_3 = &mesh.tvertices[(idx + 2) as usize];

                    //TODO(dustin) correct the bounds
//                    if !(pixel_pos.x > 0_f32 && pixel_pos.x < ww as f32 && pixel_pos.y > 0_f32 && pixel_pos.y < wh as f32) {
//                        continue;
//                    }
                    render_context.draw_triangle_v(vert_t_1, vert_t_2, vert_t_3, &tex, &transformation_matrix);
//                    render_context.draw_triangle1(vert1, vert2, vert3, &tex); //top left right

                    let data = render_context.get_raw();

                    {
                        let color = Color::rgba(200, 0, 0, 255);
                        let new = color.data;
                        let old = &mut data[vert1.y as usize * ww as usize + vert1.x as usize].data;
                        *old = new;
                    }


                    {
                        let color = Color::rgba(0, 200, 0, 255);
                        let new = color.data;
                        let old = &mut data[vert2.y as usize * ww as usize + vert2.x as usize].data;
                        *old = new;
                    }

                    {
                        let color = Color::rgba(0, 0, 200, 255);
                        let new = color.data;
                        let old = &mut data[vert3.y as usize * ww as usize + vert3.x as usize].data;
                        *old = new;
                    }
                }
            }
        }

        render_context.sync();

        frame_cnt += 1_f32;
        counter_duration += delta_ms;
        if counter_duration > 1000_f32 {
            println!("FPS: {}", frame_cnt / counter_duration * 1000_f32);
            frame_cnt = 0_f32;
            counter_duration = 0_f32;
        }

        //handle events
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