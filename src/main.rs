#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use rand::Rng;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::{FPoint, FRect, WindowCanvas};
use std::time::Duration;

fn get_food_cords() -> Vec<i8> {
    vec![
        rand::rng().random_range(0..10),
        rand::rng().random_range(0..10),
        rand::rng().random_range(0..10),
    ]
}

const OFFSET: FPoint = FPoint { x: 50.0, y: 50.0 };

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
    let mut move_vector = [1, 0, 0];

    let base_cube = ColoredCube::new(
        50.0,
        50.0,
        500.0,
        500.0,
        perspective,
        Color::RGB(0, 0, 0),
        false,
    );

    // let mut cubes: Vec<ColoredCube> = Vec::new();
    let cube_side = 30.0;
    let cube_offset = 3.0;
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    let move_speed = 30;

    let mut cubes: Vec<ColoredCube> = Vec::new();

    cubes.push(ColoredCube::virtual_3d(
        x as f32 * (cube_side + cube_offset),
        y as f32 * (cube_side + cube_offset),
        z as f32 * (cube_side + cube_offset),
        cube_side,
        cube_side,
        small_perspective,
        Color::RGB(255, 255, 255),
        false,
        OFFSET,
    ));

    let mut food_pos = get_food_cords();
    let mut food = ColoredCube::virtual_3d(
        food_pos[0] as f32 * (cube_side + cube_offset),
        food_pos[1] as f32 * (cube_side + cube_offset),
        food_pos[2] as f32 * (cube_side + cube_offset),
        cube_side,
        cube_side,
        small_perspective,
        Color::RGB(0, 0, 255),
        false,
        OFFSET,
    );

    'running: loop {
        i += 1;
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    move_vector = [0, -1, 0];
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    move_vector = [1, 0, 0];
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    move_vector = [0, 0, -1];
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    move_vector = [0, 1, 0];
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    move_vector = [-1, 0, 0];
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    move_vector = [0, 0, 1];
                }
                _ => {}
            }
        }

        if i % move_speed == 0 {
            x += move_vector[0];
            y += move_vector[1];
            z += move_vector[2];

            cubes.push(ColoredCube::virtual_3d(
                x as f32 * (cube_side + cube_offset),
                y as f32 * (cube_side + cube_offset),
                z as f32 * (cube_side + cube_offset),
                cube_side,
                cube_side,
                small_perspective,
                Color::RGB(255, 255, 255),
                false,
                OFFSET,
            ));
            if x == food_pos[0] && y == food_pos[1] && z == food_pos[2] {
                food_pos = get_food_cords();
                food = ColoredCube::virtual_3d(
                    food_pos[0] as f32 * (cube_side + cube_offset),
                    food_pos[1] as f32 * (cube_side + cube_offset),
                    food_pos[2] as f32 * (cube_side + cube_offset),
                    cube_side,
                    cube_side,
                    small_perspective,
                    Color::RGB(0, 0, 255),
                    false,
                    OFFSET,
                );
            } else {
                println!("food position {:?}", food_pos);
                println!("head position {:?}", [x, y, z]);
                cubes.remove(0);
            }
        }

        base_cube.draw(&mut canvas);

        for cube in &cubes {
            cube.draw(&mut canvas);
        }

        food.draw(&mut canvas);

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
        let base_rect = FRect::new(self.x, self.y, self.w, self.h);
        canvas.draw_rect(base_rect).unwrap();
        if self.fill {
            canvas.fill_rect(base_rect).unwrap();
        }
        Self::draw_perspective_lines(self.perspective, canvas, base_rect, self.color);
    }
}
