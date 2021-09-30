use skia_safe::{Color, Data, EncodedImageFormat, Paint, PaintStyle, Path, Surface, Rect};
use std::mem;
use std::fs::File;
use std::io::Write;

fn main() {
    draw_circle_symbol();
    draw_healthbar_texture();
}

fn draw_healthbar_texture() {
    let mut canvas = Canvas::new(2, 1);
    //canvas.set_line_width(line_width);
    //canvas.stroke();
    canvas.set_color(Color::GREEN);
    canvas.draw_rectangle(0.0, 0.0, 1.0, 1.0);
    canvas.set_color(Color::RED);
    canvas.draw_rectangle(1.0, 0.0, 1.0, 1.0);

    let d = canvas.data();
    let mut file = File::create("assets/textures/healthbar.png").unwrap();
    let bytes = d.as_bytes();
    file.write_all(bytes).unwrap();
}

fn draw_circle_symbol() {
    let base = 512f32;
    let line_width = 5.0;
    let mut canvas = Canvas::new(base as i32, base as i32);
    canvas.set_line_width(line_width);
    canvas.stroke();
    canvas.draw_circle(base/2.0, base/2.0, (base-line_width)/2.0);

    let d = canvas.data();
    let mut file = File::create("assets/textures/unit_overlays/test.png").unwrap();
    let bytes = d.as_bytes();
    file.write_all(bytes).unwrap();
}

pub struct Canvas {
    surface: Surface,
    path: Path,
    paint: Paint,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Canvas {
        let mut surface = Surface::new_raster_n32_premul((width, height)).expect("no surface!");
        let path = Path::new();
        let mut paint = Paint::default();
        paint.set_color(Color::WHITE);
        paint.set_anti_alias(true);
        paint.set_stroke_width(1.0);
        surface.canvas().clear(Color::WHITE.with_a(0u8));
        Canvas {
            surface,
            path,
            paint,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.paint.set_color(color);
    }

    pub fn save(&mut self) {
        self.canvas().save();
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.canvas().translate((dx, dy));
    }

    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.canvas().scale((sx, sy));
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.begin_path();
        self.path.move_to((x, y));
    }

    pub fn line_to(&mut self, x: f32, y: f32) {
        self.path.line_to((x, y));
    }

    pub fn quad_to(&mut self, cpx: f32, cpy: f32, x: f32, y: f32) {
        self.path.quad_to((cpx, cpy), (x, y));
    }

    pub fn draw_circle(&mut self, x: f32, y: f32, radius: f32) {
        let paint = &self.paint.clone();
        self.canvas().draw_circle((x, y), radius, paint);
    }

    pub fn draw_rectangle(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let paint = &self.paint.clone();
        self.canvas().draw_rect(Rect::from_xywh(x, y, w, h), paint);
    }

    pub fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
        self.path.cubic_to((cp1x, cp1y), (cp2x, cp2y), (x, y));
    }

    pub fn close_path(&mut self) {
        self.path.close();
    }

    pub fn begin_path(&mut self) {
        let new_path = Path::new();
        self.surface.canvas().draw_path(&self.path, &self.paint);
        let _ = mem::replace(&mut self.path, new_path);
    }

    pub fn stroke(&mut self) {
        self.paint.set_style(PaintStyle::Stroke);
        self.surface.canvas().draw_path(&self.path, &self.paint);
    }

    pub fn fill(&mut self) {
        self.paint.set_style(PaintStyle::Fill);
        self.surface.canvas().draw_path(&self.path, &self.paint);
    }

    pub fn set_line_width(&mut self, width: f32) {
        self.paint.set_stroke_width(width);
    }

    pub fn data(&mut self) -> Data {
        let image = self.surface.image_snapshot();
        image.encode_to_data(EncodedImageFormat::PNG).unwrap()
    }

    fn canvas(&mut self) -> &mut skia_safe::Canvas {
        self.surface.canvas()
    }
}