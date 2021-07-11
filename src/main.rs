use macroquad::prelude::*;

#[derive(Copy, Clone)]
struct Dot {
    x: f32,
    y: f32,
}

impl Dot {
    fn new(x: f32, y: f32) -> Dot {
        Dot { x, y }
    }

    fn draw(&self, color: Color) {
        draw_circle(self.x, self.y, 10.0, color); // 3.0
    }

    fn nearest_dot(&self, dots: &Vec<Dot>) -> Dot {
        // TODO: process Vec with 0 lenght.
        assert_ne!(dots.len(), 0);

        let mut min_distance = distance_between_dots(&self, &dots[0]);
        let mut out_dot = dots[0];
        for dot in dots.iter() {
            let current_distance = distance_between_dots(&self, dot);
            if current_distance < min_distance {
                min_distance = current_distance;
                out_dot = *dot;
            }
        }
        out_dot
    }

    fn get_rotated_relative_dot(&self, dot: &Dot, angle: f32) -> Dot {
        let x = dot.x + (self.x - dot.x) * angle.cos() - (self.y - dot.y) * angle.sin();
        let y = dot.y + (self.x - dot.x) * angle.sin() + (self.y - dot.y) * angle.cos();
        Dot::new(x, y)
    }

    fn is_equal(&self, dot: Dot) -> bool {
        dot.x == self.x && dot.y == self.y
    }
}

pub struct Line {
    a: f32,
    b: f32,
    c: f32,
    from: Option<Dot>,
    to: Option<Dot>,
}

impl Line {
    fn new_from_abc(a: f32, b: f32, c: f32) -> Line {
        Line {a, b, c, from: None, to: None}
    }

    fn new(from: &Dot, to: &Dot) -> Line {
        let (a, b, c) = Line::get_abc_from_dots(from, to);
        
        Line { a, b, c, from: Some(*from), to: Some(*to)}
    }

    fn get_abc_from_dots(from: &Dot, to: &Dot) -> (f32, f32, f32) {
        let (x1, x2) = (from.x, to.x);
        let (y1, y2) = (from.y, to.y);
        let mut a = y1 - y2;
        let mut b = x2 - x1;
        let mut c = x1 * y2 - x2 * y1;
        
        if a != 0.0 && b != 0.0 && c != 0.0{
            b /= a.abs();
            c /= a.abs();
            a /= a.abs();
        }

        (a, b, c)
    }

    fn get_abc_f64_from_dots(from: &Dot, to: &Dot) -> (f64, f64, f64) {
        let (a, b, c) = Line::get_abc_from_dots(from, to);
        (a as f64, b as f64, c as f64)
    }

    fn get_abc(&self) -> (f32, f32, f32){
        (self.a, self.b, self.c)
    }

    fn get_abc_f64(&self) -> (f64, f64, f64) {
        (self.a as f64, self.b as f64, self.c as f64)
    }

    fn get_moved_to_dot_line(&self, dot: Dot) -> Line {
        let c = -self.a * dot.x - self.b * dot.y;
        //Line::new_from_abc(self.a, self.b, c)
        todo!();
        Line { a: self.a, b: self.b, c, from: Some(dot), to: None }
    }

    // fn move_to_dot let c = -self.a * dot.x - self.b * dot.y

    fn get_angle(&self) -> f32 {
        let cos = self.b / (self.a.powi(2) + self.b.powi(2)).sqrt();
        cos.acos()
    }

    fn get_angle_with_line(&self, line: &Line) -> f32 {
        let cos = (self.a * line.a + self.b * line.b) / ((self.a.powi(2) + self.b.powi(2)).sqrt() * (line.a.powi(2) + line.b.powi(2)).sqrt());
        cos.acos()
    }

    fn get_rotated_line_by_angle(&self, angle: f32) -> Line {
        let (from, to) = self.get_from_to();
        let new_to = to.get_rotated_relative_dot(&from, -angle);
        Line::new(&from, &new_to)
    }

    /*fn get_dot_from_x(&self, x: f32) -> Dot {
        assert_ne!(self.b, 0.0);
        let y = (-self.a*x - self.c) / self.b;
        Dot::new(x, y)
    }*/

    fn get_dot_from_y(&self, y: f32) -> Dot {
        assert_ne!(self.a, 0.0);
        let x = (-self.b*y - self.c) / self.a;
        Dot::new(x, y)
    }

    fn get_default_from_to(&self) -> (Dot, Dot){
        let from: Dot;
        let to: Dot;
        if self.a == 0.0 {
            let y = -self.c / self.b;
            from = Dot::new(0.0, y);
            to = Dot::new(screen_width(), y);
        }
        else
        {
            if self.a < 0.0 {
                from = self.get_dot_from_y(0.0);
                to = self.get_dot_from_y(screen_height());
            }
            else {
                to = self.get_dot_from_y(0.0);
                from = self.get_dot_from_y(screen_height());
            }
            
        }

        (from, to)
    }

    fn get_from_to(&self) -> (Dot, Dot) {
        let from: Dot;
        let to: Dot;
        let (default_from, default_to) = self.get_default_from_to();
        match self.from {
            Some(x) => from = x,
            None => from = default_from,
        }

        match self.to {
            Some(x) => to = x,
            None => to = default_to,
        }
        (from, to)
    }

    fn get_invert_line(&self) -> Line {
        let (from, to) = self.get_from_to();
        Line::new(&to, &from)
    }

    fn is_dot_have_same_range(&self, dot: Dot) -> bool {
        let (from, to) = self.get_from_to();
        from.x <= dot.x && from.y >= dot.y && to.x >= dot.x && to.y <= dot.y
    }

    fn draw(&self) {
        let (from, to) = self.get_default_from_to();
        draw_line(from.x, from.y, to.x, to.y, 2.0, BLACK);
    }

    fn draw_with_bounds(&self) {
        let (from, to) = self.get_from_to();
        draw_line(from.x, from.y, to.x, to.y, 2.0, BLACK);
    }

    fn draw_with_to_bound(&self) {
        let (from, _) = self.get_from_to();
        let (_, to) = self.get_default_from_to();
        draw_line(from.x, from.y, to.x, to.y, 2.0, BLACK);
    }
}

struct Circle {
    center: Dot,
    r: f32,
}

impl Circle {
    fn new(center: Dot, r: f32) -> Circle {
        Circle {center, r}
    }

    fn draw(&self) {
        draw_poly_lines(self.center.x, self.center.y, 255, self.r, 2.0*std::f32::consts::PI, 2.0, BLACK);
    }

    fn get_tangent_line(&self, dot: Dot) -> Line {
        let radius_line = Line::new(&self.center, &dot);
        let tangent = radius_line.get_invert_line().get_rotated_line_by_angle(std::f32::consts::PI / 2.0);
        tangent
    }
}


/*
pub mod graphics {
    use crate::Line;

    pub fn draw_line(line: Line) {
        let (x1, x2) = (line.from.x, line.to.x);
        let (y1, y2) = (line.from.y, line.to.y);
        draw_line(x1, y1, x2, y2, 5.0, BLACK);
    }
}*/

fn line_circle_intersection(line: &Line, circle: &Circle) -> Vec<Dot> {
    let mut out: Vec<Dot> = Vec::new();
    let (a, b, c) = line.get_abc_f64();
    let (x, y, r) = (circle.center.x as f64, circle.center.y as f64, circle.r as f64);

    let a_y = a.powi(2) + b.powi(2);
    let b_y = 2.0*b*c + 2.0*a*x*b - 2.0*a.powi(2)*y;
    let c_y = c.powi(2) + 2.0*a*x*c + a.powi(2)*x.powi(2) + a.powi(2)*y.powi(2) - a.powi(2) * r.powi(2);
    let discr_y = b_y.powi(2) - 4.0*a_y*c_y;

    let a_x = a.powi(2) + b.powi(2);
    let b_x = 2.0*a*c + 2.0*b*y*a - 2.0*b.powi(2)*x;
    let c_x = b.powi(2)*x.powi(2) + c.powi(2) + 2.0*b*y*c + y.powi(2)*b.powi(2) - r.powi(2)*b.powi(2);
    let discr_x = b_x.powi(2) - 4.0*a_x*c_x;

    if is_mouse_button_pressed(MouseButton::Left) {
        println!("discr_y: {}, discr_x: {}", discr_y, discr_x);
    }
    
    let y1: f64;
    let y2: f64;
    let x1: f64;
    let x2: f64;
    if discr_y > 0.0 && discr_x > 0.0 {
        if a != 0.0 {
            y1 = (-b_y + discr_y.sqrt()) / (2.0*a_y);
            y2 = (-b_y - discr_y.sqrt()) / (2.0*a_y);
            x1 = (-b*y1 - c) / a;
            x2 = (-b*y2 - c) / a;
        }
        else {
            x1 = (-b_x + discr_x.sqrt()) / (2.0*a_x);
            x2 = (-b_x - discr_x.sqrt()) / (2.0*a_x);
            y1 = (-a*x1 - c) / b;
            y2 = (-a*x2 - c) / b;
        }
        let d1 = Dot::new(x1 as f32, y1 as f32);
        let d2 = Dot::new(x2 as f32, y2 as f32);

        if line.is_dot_have_same_range(d1) {
            out.push(d1);
        }
        
        if line.is_dot_have_same_range(d2) {
            out.push(d2);
        }
    }
    else if discr_x >= 0.0 && discr_y >= 0.0 {
        y1 = -b_y / (2.0*a_y);
        x1 = -b_x / (2.0*a_x);
        let d1 = Dot::new(x1 as f32, y1 as f32);
        if line.is_dot_have_same_range(d1) {
            out.push(d1);
        }
    }

    out
}

fn distance_between_dots(d1: &Dot, d2: &Dot) -> f32{
    let square = (d1.x - d2.x).powi(2) + (d1.y - d2.y).powi(2);
    square.sqrt()
}

fn example1(cursor: Dot, debug: bool) {
    let circle = Circle::new(Dot::new(400.0, 300.0), 100.0);
    circle.draw();

    let from_line_dot = Dot::new(100.0, 400.0);
    let line = Line::new(&from_line_dot, &cursor);

    let dots: Vec<Dot> = line_circle_intersection(&line, &circle);

    for dot in &dots {
        dot.draw(BLUE);
    }

    if dots.len() > 0 {
        let near_dot = from_line_dot.nearest_dot(&dots);
        near_dot.draw(RED);
        let line_to_circle = Line::new(&from_line_dot, &near_dot);
        line_to_circle.draw_with_bounds();
        let radius_line = Line::new(&circle.center, &near_dot);
        radius_line.draw_with_bounds();
        let tangent = circle.get_tangent_line(near_dot);
        
        let target_angle = line_to_circle.get_angle_with_line(&tangent);

        let reflected_line = tangent.get_rotated_line_by_angle(target_angle);
        reflected_line.draw_with_to_bound();

        if debug {
            println!("angle: {}", reflected_line.get_angle() * 180.0 / std::f32::consts::PI);
        }
    }
    else {
        line.draw_with_bounds();
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        println!("x: {}, y: {}.\n", cursor.x, cursor.y);
        println!("line: {:?}", line.get_abc());
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "2d".to_owned(),
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    // let mut angle = std::f32::consts::PI / 4.0;
    loop {
        clear_background(WHITE);
        let debug = is_mouse_button_pressed(MouseButton::Left);
 
        let (x, y): (f32, f32) = mouse_position();
        let cursor = Dot { x, y };
        cursor.draw(BLACK);

        /*
        let (_, b) = mouse_wheel();
        angle += (std::f32::consts::PI / 100.0) * b;
        let from_line_dot = Dot::new(100.0, 400.0);
        let line = Line::new(&from_line_dot, &cursor);
        line.draw_with_bounds();
        let line = line.get_rotated_line_by_angle(angle);
        line.draw_with_bounds(); */

        example1(cursor, debug);
        
        draw_text(&format!("IT WORKS! FPS: {}.", macroquad::time::get_fps())[..], 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
