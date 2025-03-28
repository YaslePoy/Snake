#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::{FPoint, FRect, WindowCanvas};
use std::time::Duration;
use rand::Rng;

fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl3 demo", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let perspective = FPoint::new(150.0, 150.0);
    let small_perspective = FPoint::new(15.0, 15.0);

    let base_cube = ColoredCube::new(
        50.0,
        50.0,
        500.0,
        500.0,
        perspective,
        Color::RGB(0, 0, 0),
        false,
    );

    let mut cubes: Vec<ColoredCube> = Vec::new();
    let cube_side = 30.0;
    let cube_offset = 3.0;

    for x in 0..5 {
        for y in 0..5 {
            for z in 0..5 {
                let random = rand::rng().random::<bool>();
                if random {
                    continue;
                }
                cubes.push(ColoredCube::virtual_3d(
                    x as f32 * (cube_side + cube_offset),
                    y as f32 * (cube_side + cube_offset),
                    z as f32 * (cube_side + cube_offset),
                    cube_side,
                    cube_side,
                    small_perspective,
                    Color::RGB(255, 255, 255),
                    false,
                    FPoint::new(50.0, 50.0),
                ));
            }
        }
    }

    // let cursor_cube = ColoredCube::virtual_3d(
    //     30.0,
    //     10.0,
    //     100.0,
    //     30.0,
    //     30.0,
    //     small_perspective,
    //     Color::RGB(255, 255, 255),
    //     false,
    //     FPoint::new(50.0, 50.0),
    // );

    // cubes.push(cursor_cube);

    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseMotion { xrel, yrel, .. } => {

                }
                _ => {}
            }
        }

        base_cube.draw(&mut canvas);

        for cube in &mut cubes {
            cube.draw(&mut canvas);
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

struct ColoredCube {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub perspective: FPoint,
    pub color: Color,
    pub fill: bool,
    rect: FRect,
}

impl ColoredCube {
    pub fn new(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        perspective: FPoint,
        color: Color,
        fill: bool,
    ) -> ColoredCube {
        ColoredCube {
            x,
            y,
            w,
            h,
            perspective,
            color,
            fill,
            rect: FRect::new(x, y, w, h),
        }
    }

    pub fn virtual_3d(
        x: f32,
        y: f32,
        z: f32,
        w: f32,
        h: f32,
        perspective: FPoint,
        color: Color,
        fill: bool,
        offset: FPoint,
    ) -> ColoredCube {
        let len = (perspective.x * perspective.x + perspective.y * perspective.y).sqrt();
        ColoredCube {
            x: x + offset.x + perspective.x * z / len,
            y: y + offset.y + perspective.y * z / len,
            w,
            h,
            perspective,
            color,
            fill,
            rect: FRect::new(
                x + offset.x + perspective.x * z / len,
                y + offset.y + perspective.y * z / len,
                w,
                h,
            ),
        }
    }
    fn draw_perspective_lines(
        presp: FPoint,
        canvas: &mut WindowCanvas,
        background: FRect,
        color: Color,
    ) {
        canvas.set_draw_color(color);
        canvas
            .draw_line(
                FPoint::new(background.x, background.y),
                FPoint::new(background.x + presp.x, background.y + presp.y),
            )
            .unwrap();
        canvas
            .draw_line(
                FPoint::new(background.x + background.w, background.y),
                FPoint::new(
                    background.x + background.w + presp.x,
                    background.y + presp.y,
                ),
            )
            .unwrap();
        canvas
            .draw_line(
                FPoint::new(background.x, background.y + background.h),
                FPoint::new(
                    background.x + presp.x,
                    background.y + background.h + presp.y,
                ),
            )
            .unwrap();
        canvas
            .draw_line(
                FPoint::new(background.x + background.w, background.y + background.h),
                FPoint::new(
                    background.x + background.w + presp.x,
                    background.y + background.h + presp.y,
                ),
            )
            .unwrap();

        let mut front_rect = background.clone();
        front_rect.x += presp.x;
        front_rect.y += presp.y;

        canvas.set_draw_color(color);
        canvas.draw_rect(front_rect).unwrap()
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(self.color);
        canvas.draw_rect(self.rect).unwrap();
        if self.fill {
            canvas.fill_rect(self.rect).unwrap();
        }
        Self::draw_perspective_lines(self.perspective, canvas, self.rect, self.color);
    }
}
