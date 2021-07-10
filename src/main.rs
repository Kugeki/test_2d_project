use macroquad::prelude::*;

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
}

pub struct Line {
    from: Dot,
    to: Dot,
}

impl Line {
    fn new(from: Dot, to: Dot) -> Line {
        Line { from, to }
    }

    fn get_abc(&self) -> (f32, f32, f32) {
        let (x1, x2) = (self.from.x, self.to.x);
        let (y1, y2) = (self.from.y, self.to.y);
        let mut a = y1 - y2;
        let mut b = x2 - x1;
        let mut c = x1 * y2 - x2 * y1;

        if a != 0.0 && b != 0.0 && c != 0.0{
            b /= a;
            c /= a;
            a = 1.0;
        }

        (a, b, c)
    }

    fn get_abc_f64(&self) -> (f64, f64, f64) {
        let (a, b, c) = self.get_abc();
        (a as f64, b as f64, c as f64)
    }

    fn draw(&self) {
        draw_line(self.from.x, self.from.y, self.to.x, self.to.y, 2.0, BLACK);
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
        out.push(Dot::new(x1 as f32, y1 as f32));
        out.push(Dot::new(x2 as f32, y2 as f32));
    }
    else if discr_x >= 0.0 && discr_y >= 0.0 {
        y1 = -b_y / (2.0*a_y);
        x1 = -b_x / (2.0*a_x);
        out.push(Dot::new(x1 as f32, y1 as f32));
    }

    out
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
    loop {
        clear_background(WHITE);
 
        let (x, y): (f32, f32) = mouse_position();
        let cursor = Dot { x, y };
        cursor.draw(BLACK);

        let circle = Circle::new(Dot::new(400.0, 300.0), 100.0);
        circle.draw();

        let line = Line::new(Dot::new(100.0, 400.0), Dot::new(x, y));
        line.draw();

        let dots: Vec<Dot> = line_circle_intersection(&line, &circle);

        for dot in &dots {
            dot.draw(BLUE);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            println!("x: {}, y: {}.\n", x, y);
            println!("line: {:?}", line.get_abc());
        }
        draw_text(&format!("IT WORKS! FPS: {}.", macroquad::time::get_fps())[..], 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
