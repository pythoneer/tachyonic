#![feature(step_by)]

extern crate orbclient;
extern crate nalgebra as na;

use orbclient::EventIter;
use orbclient::Renderer;
use orbclient::color::Color;
use na::{ Vector3, Vector2, Point2, Point4, Point3, Matrix4 };

pub struct RenderContext {
    window: Box<orbclient::Window>,
    zbuffer: Vec<f32>,
}




//x0-x1 input range y0-y1 output range : x input cursor
pub fn lerp(x0: f32, x1: f32, y0: f32, y1: f32, x: f32) -> f32 {
    y0 + (x - x0) * ((y1 - y0) / (x1 - x0))
}

//AOS <-> SOA ?
pub struct Vertex {
    pub position: Point4<f32>,
    pub texture_coordinate: Point2<f32>,
    pub color: Color,
}

impl Vertex {
    pub fn new(position: Point4<f32>, texture_coordinate: Point2<f32>, color: Color) -> Self {
        Vertex {
            position: position,
            texture_coordinate: texture_coordinate,
            color: color,
        }
    }
}

pub struct Mesh {
    pub vertices: Vec<Point4<f32>>,
    pub tvertices: Vec<Vertex>,
    pub position: Point3<f32>,
    pub rotation: Vector3<f32>,
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            vertices: Vec::new(),
            tvertices: Vec::new(),
            position: Point3::new(0_f32, 0_f32, 0_f32),
            rotation: Vector3::new(0_f32, 0_f32, 0_f32),
        }
    }
}

pub struct Texture {

}

impl Texture {

    pub fn new() -> Self {
        Texture{}
    }

    pub fn get(&self, u: f32, v: f32) -> Color {

        if u == 0.5 && v == 0.0 {
            Color::rgba(255, 0, 0, 255)
        } else if u == 0.0 && v == 1.0 {
            Color::rgba(0, 255, 0, 255)
        } else {
            Color::rgba(0, 0, 255, 255)
        }

    }
}


fn project(point: Point4<f32>, mat: &Matrix4<f32>, ww: u32, wh: u32) -> Point2<f32> {

    //transform into screenspace
    let mut screen_space_point = *mat * point;

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














impl RenderContext {
    pub fn new(width: u32, height: u32, title: &str) -> RenderContext {
        let orb_window = Box::new(orbclient::Window::new_flags(100, 100, width, height, title, true).unwrap());
        RenderContext{window: orb_window, zbuffer: vec![std::f32::MAX; (width * height) as usize] }
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.window.pixel(x, y, color);
    }

    pub fn data_mut(&mut self) -> &mut [Color] {
        self.window.data_mut()
    }

    pub fn edge_function_i(a: Vector3<i32>, b: Vector3<i32>, c: Vector2<i32>) -> bool {
        let m = (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x);
        m < 0_i32
    }

    pub fn edge_function(a: Point2<f32>, b: Point2<f32>, c: Point2<f32>) -> bool {
        let m = (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x);
        m < 0_f32
    }

    //why is area times two opposite sign?
    pub fn area_times_two(a: Point2<f32>, b: Point2<f32>, c: Point2<f32>) -> f32 {
        -((c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x))
    }

    pub fn get_raw(&mut self) -> &mut [Color] {
        self.window.data_mut()
    }

    pub fn clear(&mut self) {
        let color = Color::rgba(10, 30, 30, 255);
        let ww = self.window.width();
        let wh = self.window.height();
        self.window.rect(0, 0, ww, wh, color)
    }

//    pub fn draw_triangle3(&mut self, top: Vector3<f32>, left: Vector3<f32>, right: Vector3<f32>) {
//
//        //vertices in screen space
//        let ww_f = self.window.width() as f32;
//        let wh_f  = self.window.height() as f32;
//        let screen_top = Vector3::new((ww_f * top.x as f32) as i32, (wh_f * top.y as f32) as i32, 1);
//        let screen_left = Vector3::new((ww_f * left.x as f32) as i32, (wh_f * left.y as f32) as i32, 1);
//        let screen_right = Vector3::new((ww_f * right.x as f32) as i32, (wh_f * right.y as f32) as i32, 1);
//
//        //bounding box of triangle
//        let top_bound = (screen_top.y) as i32;
//        let bottom_bound = (std::cmp::max(screen_left.y, screen_right.y)) as i32;
//        let mut left_bound = (screen_left.x) as i32;
//        let right_bound = (screen_right.x) as i32;
//
//        //sample points top left, top right, bottom left, bottom right
//        let step = 4;
//        let mut tl = Vector2::new(0, 0);
//        let mut tr = Vector2::new(0, 0);
//        let mut bl = Vector2::new(0, 0);
//        let mut br = Vector2::new(0, 0);
//
//        let data = self.data_mut();
//
//        for y in (top_bound..bottom_bound).step_by(step) {
//
//            tl.y = y;
//            tr.y = y;
//            bl.y = y + step;
//            br.y = y + step;
//
//            let mut drawn = false;
//
//            for x in (left_bound..right_bound).step_by(step) {
//
//                tl.x = x;
//                tr.x = x + step;
//                bl.x = x;
//                br.x = x + step;
//
//                //check if all 4 points are inside
//                let mut tl_inside = true;
//                tl_inside &= RenderContext::edge_function_i(screen_left, screen_top, tl);
//                tl_inside &= RenderContext::edge_function_i(screen_top, screen_right, tl);
//                tl_inside &= RenderContext::edge_function_i(screen_right, screen_left, tl);
//
//                let mut tr_inside = true;
//                tr_inside &= RenderContext::edge_function_i(screen_left, screen_top, tr);
//                tr_inside &= RenderContext::edge_function_i(screen_top, screen_right, tr);
//                tr_inside &= RenderContext::edge_function_i(screen_right, screen_left, tr);
//
//                let mut bl_inside = true;
//                bl_inside &= RenderContext::edge_function_i(screen_left, screen_top, bl);
//                bl_inside &= RenderContext::edge_function_i(screen_top, screen_right, bl);
//                bl_inside &= RenderContext::edge_function_i(screen_right, screen_left, bl);
//
//                let mut br_inside = true;
//                br_inside &= RenderContext::edge_function_i(screen_left, screen_top, br);
//                br_inside &= RenderContext::edge_function_i(screen_top, screen_right, br);
//                br_inside &= RenderContext::edge_function_i(screen_right, screen_left, br);
//
//                let inside = tl_inside && tr_inside && bl_inside && br_inside;
//                let edge = tl_inside || tr_inside || bl_inside || br_inside;
//
//                //fill entire square
//                if inside {
//
//                    //do not check before triangle .. skip from left
////                    if !drawn {
////                        left_bound = x / 2;
////                    }
//
//                    let color = Color::rgba((x / 6) as u8, (y / 6) as u8, 100, 255);
//                    for sy in y..y+step as i32 {
//                        for sx in x..x+step as i32 {
//                            let new = color.data;
//                            let old = &mut data[sy as usize * ww_f as usize + sx as usize].data;
//                            *old = new;
////                            self.draw_pixel(sx, sy, Color::rgba((x / 6) as u8, (y / 6) as u8, 100, 255));
//
//                        }
//                    }
//
//                    //                    self.window.rect(x, y, step as u32, step as u32, Color::rgba(100, 100, 250, 255));
//                    drawn = true
//
//                } else if edge {
//                    for sy in  y..y+step as i32 {
//                        for sx in x..x+step as i32 {
//                            let p = Vector2::new(sx, sy);
//
//                            let mut p_inside = true;
//                            p_inside &= RenderContext::edge_function_i(screen_left, screen_top, p);
//                            p_inside &= RenderContext::edge_function_i(screen_top, screen_right, p);
//                            p_inside &= RenderContext::edge_function_i(screen_right, screen_left, p);
//
//                            if p_inside {
//                                let color = Color::rgba(240, 100, 100, 255);
////                                self.draw_pixel(sx, sy, Color::rgba(240, 100, 100, 255));
//                                let new = color.data;
//                                let old = &mut data[sy as usize * ww_f as usize + sx as usize].data;
//                                *old = new;
//                            }
//                        }
//                    }
//                } else {
//                    //fully outside no worries
////                    let color = Color::rgba(100, 100, 250, 255);
////                    let new = color.data;
////                    let old = &mut data[y as usize * ww_f as usize + x as usize].data;
////                    *old = new;
//
//                    //do not check after triangle on the right
//                    if drawn  {
//                        break;
//                    }
//                }
//
//
//            }
//        }
//
//    }
//
    pub fn draw_triangle2(&mut self, top: Point2<f32>, left: Point2<f32>, right: Point2<f32>) {

        let step = 4;

        let ww = self.window.width();
        let wh = self.window.height();

        //screen space
        let top_bound = (top.y) as i32;
        let bottom_bound = (f32::max(left.y, right.y)) as i32;
        let mut left_bound = (left.x) as i32;
        let right_bound = (right.x) as i32;

        //world space
//        let mut tl = Point2::new(left_bound as f32 / self.window.width() as f32, top_bound as f32 / self.window.height() as f32);
//        let mut tr = Point2::new(left_bound as f32 / self.window.width() as f32 + step as f32 / self.window.width() as f32, top_bound as f32 / self.window.height() as f32);
//        let mut bl = Point2::new(left_bound as f32 / self.window.width() as f32, top_bound as f32 / self.window.height() as f32 + step as f32 / self.window.width() as f32);
//        let mut br = Point2::new(left_bound as f32 / self.window.width() as f32 + step as f32 / self.window.width() as f32, top_bound as f32 / self.window.height() as f32+ step as f32 / self.window.width() as f32);

        let mut tl = Point2::new(0.0, 0.0);
        let mut tr = Point2::new(0.0, 0.0);
        let mut bl = Point2::new(0.0, 0.0);
        let mut br = Point2::new(0.0, 0.0);


        let data = self.data_mut();

        for y in (top_bound..bottom_bound).step_by(step) {

            tl.y = y as f32;
            tr.y = y as f32;
            bl.y = y as f32 + step as f32;
            br.y = y as f32 + step as f32;

//            let mut drawn = false;

            for x in (left_bound..right_bound).step_by(step) {

                tl.x = x as f32;
                tr.x = x as f32 + step as f32;
                bl.x = x as f32;
                br.x = x as f32 + step as f32;

                //check if all 4 points are inside
                let mut tl_inside = true;
                tl_inside &= RenderContext::edge_function(left, top, tl);
                tl_inside &= RenderContext::edge_function(top, right, tl);
                tl_inside &= RenderContext::edge_function(right, left, tl);

                let mut tr_inside = true;
                tr_inside &= RenderContext::edge_function(left, top, tr);
                tr_inside &= RenderContext::edge_function(top, right, tr);
                tr_inside &= RenderContext::edge_function(right, left, tr);

                let mut bl_inside = true;
                bl_inside &= RenderContext::edge_function(left, top, bl);
                bl_inside &= RenderContext::edge_function(top, right, bl);
                bl_inside &= RenderContext::edge_function(right, left, bl);

                let mut br_inside = true;
                br_inside &= RenderContext::edge_function(left, top, br);
                br_inside &= RenderContext::edge_function(top, right, br);
                br_inside &= RenderContext::edge_function(right, left, br);

                let inside = tl_inside && tr_inside && bl_inside && br_inside;
                let edge = tl_inside || tr_inside || bl_inside || br_inside;

                //fill entire square
                if inside {
//                    let color = Color::rgba(100, 100, 100, 255);
                    for sy in y..y+step as i32 {
                        for sx in x..x+step as i32 {
//                            self.draw_pixel(sx, sy, Color::rgba((x / 6) as u8, (y / 6) as u8, 100, 255));
                            let color = Color::rgba((x / 6) as u8, (y / 6) as u8, 100, 255);
                            let new = color.data;
                            let old = &mut data[sy as usize * ww as usize + sx as usize].data;
                            *old = new;
                        }
                    }

//                    self.window.rect(x, y, step as u32, step as u32, Color::rgba(100, 100, 250, 255));

//                    drawn = true
                } else if edge {
                    for sy in  y..y+step as i32 {
                        for sx in x..x+step as i32 {
                            let p = Point2::new(sx as f32, sy as f32);

                            let mut p_inside = true;
                            p_inside &= RenderContext::edge_function(left, top, p);
                            p_inside &= RenderContext::edge_function(top, right, p);
                            p_inside &= RenderContext::edge_function(right, left, p);

                            if p_inside {
//                                self.draw_pixel(sx, sy, Color::rgba(240, 100, 100, 255));
                                let color = Color::rgba(240, 100, 100, 255);
                                let new = color.data;
                                let old = &mut data[sy as usize * ww as usize + sx as usize].data;
                                *old = new;
                            }
                        }
                    }
                } else {
                    //fully outside no worries
//                    self.window.rect(x, y, step as u32, step as u32, Color::rgba(100, 200, 100, 255));

//                    if drawn  {
//                        break;
//                    }
                }
            }
        }
    }

    pub fn draw_triangle_v(&mut self, vtop: &Vertex, vleft: &Vertex, vright: &Vertex, texture: &Texture, transform: &Matrix4<f32>) {

        let ww = self.window.width();
        let wh = self.window.height();
        let ww_f = self.window.width() as f32;
        let wh_f  = self.window.height() as f32;

        let top = project(vtop.position, transform, ww, wh);
        let left = project(vleft.position, transform, ww, wh);
        let right = project(vright.position, transform, ww, wh);

        let top_bound = (top.y) as i32;
        let bottom_bound = (f32::max(left.y, right.y)) as i32;
        let mut left_bound = (left.x) as i32;
        let right_bound = (right.x) as i32;

        let data = self.data_mut();


        for y in top_bound..bottom_bound {

            //            let mut drawn = false;

            for x in left_bound..right_bound {


                let p = Point2::new(x as f32 , y as f32);
                let area = RenderContext::area_times_two(top, left, right); // area of the triangle multiplied by 2
                let mut w0 = RenderContext::area_times_two(left, top, p); // signed area of the triangle v1v2p multiplied by 2
                let mut w1 = RenderContext::area_times_two(top, right, p); // signed area of the triangle v2v0p multiplied by 2
                let mut w2 = RenderContext::area_times_two(right, left, p); // signed area of the triangle v0v1p multiplied by 2

                let p_inside = w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0;


//                let p = Point2::new(x as f32 , y as f32);
//                let mut p_inside = true;
//                p_inside &= RenderContext::edge_function(left, top, p);
//                p_inside &= RenderContext::edge_function(top, right, p);
//                p_inside &= RenderContext::edge_function(right, left, p);

                if p_inside {
                    //                    self.draw_pixel(x, y, Color::rgba(200, 50, 50, 255));

                    // barycentric coordinates are the areas of the sub-triangles divided by the area of the main triangle
                    w0 /= area;
                    w1 /= area;
                    w2 /= area;


//                    float r = w0 * c0[0] + w1 * c1[0] + w2 * c2[0];
//                    float g = w0 * c0[1] + w1 * c1[1] + w2 * c2[1];
//                    float b = w0 * c0[2] + w1 * c1[2] + w2 * c2[2];
                    let r = w0 * (vright.color.r() as f32) + w1 * (vleft.color.r() as f32) + w2 * (vtop.color.r() as f32);
                    let g = w0 * (vright.color.g() as f32) + w1 * (vleft.color.g() as f32) + w2 * (vtop.color.g() as f32);
                    let b = w0 * (vright.color.b() as f32) + w1 * (vleft.color.b() as f32) + w2 * (vtop.color.b() as f32);


                    let color = Color::rgba(-r as u8, -g as u8, -b as u8, 255);
//                    let u =
                    let new = color.data;
                    let old = &mut data[y as usize * ww as usize + x as usize].data;
                    *old = new;

                    //                    drawn = true;
                } else {
                    // do not check after triangle


                    //                    let color = Color::rgba(255, 150, 150, 255);
                    //                    let new = color.data;
                    //                    let old = &mut data[y as usize * ww as usize + x as usize].data;
                    //                    *old = new;

                    //                    if drawn  {
                    //                        break;
                    //                    }
                }


            }
        }

    }

    //assume vertices are in right order and in raster space
    pub fn draw_triangle1(&mut self, top: Point2<f32>, left: Point2<f32>, right: Point2<f32>, texture: &Texture) {

        let ww = self.window.width();
        let wh = self.window.height();
        let ww_f = self.window.width() as f32;
        let wh_f  = self.window.height() as f32;

        let top_bound = (top.y) as i32;
        let bottom_bound = (f32::max(left.y, right.y)) as i32;
        let mut left_bound = (left.x) as i32;
        let right_bound = (right.x) as i32;

        let data = self.data_mut();



        for y in top_bound..bottom_bound {

//            let mut drawn = false;

            for x in left_bound..right_bound {

                let p = Point2::new(x as f32 , y as f32);
                let mut p_inside = true;
                p_inside &= RenderContext::edge_function(left, top, p);
                p_inside &= RenderContext::edge_function(top, right, p);
                p_inside &= RenderContext::edge_function(right, left, p);

                if p_inside {
//                    self.draw_pixel(x, y, Color::rgba(200, 50, 50, 255));

                    let color = Color::rgba((x / 6) as u8, (y / 6) as u8, 100, 255);
                    let new = color.data;
                    let old = &mut data[y as usize * ww as usize + x as usize].data;
                    *old = new;

//                    drawn = true;
                } else {
                    // do not check after triangle


//                    let color = Color::rgba(255, 150, 150, 255);
//                    let new = color.data;
//                    let old = &mut data[y as usize * ww as usize + x as usize].data;
//                    *old = new;

//                    if drawn  {
//                        break;
//                    }
                }


            }
        }

    }

//    pub fn draw_triangleX(&mut self, _: Vector3<f32>, _: Vector3<f32>, _: Vector3<f32>) {
//
//        let color = Color::rgba(200, 150, 100, 255);
//        let ww = self.window.width();
//        let wh = self.window.height();
//        let data = self.data_mut();
//
//        for y in 0..wh {
//            for x in 0..ww {
//                let new = color.data;
//                let old = &mut data[y as usize * ww as usize + x as usize].data;
//                *old = new;
//            }
//        }
//
////        self.window.rect(0,0,ww, wh, color);
//    }


    pub fn sync(&mut self) {
        self.window.sync();
    }

    pub fn events(&mut self) -> EventIter {
        self.window.events()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
