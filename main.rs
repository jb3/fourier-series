use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::f64::consts::PI;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut points: Vec<i16> = vec![];

    let window = video_subsystem
        .window("fourier series", 1000, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut angle = 0.0f64;
    let mut n = 5;

    let mut sawtooth = false;
    let mut pause = false;

    let mut tempadd = false;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    n += 1;
                    points.clear();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if n == 1 {
                    } else {
                        n -= 1;
                        points.clear();
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    sawtooth = !sawtooth;
                    points.clear()
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => pause = !pause,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    angle -= 0.01;
                    if points.len() > 0 {
                        points.remove(0);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    angle += 0.01;
                    tempadd = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => points.clear(),
                _ => {}
            }
        }

        let mut x = 200.0;
        let mut y = 400.0;

        for i in 0..n {
            let prevx = x;
            let prevy = y;

            let e: f64;
            let radius: f64;

            if !sawtooth {
                e = (i as f64) * 2.0 + 1.0;
                radius = 80.0 * (4.0 / (e * PI));
            } else {
                e = (i as f64) + 1.0;
                radius = 80.0 * (4.0 / (e * PI));
            }

            x += radius * (e.abs() * angle).cos();
            y += radius * (e.abs() * angle).sin();

            let hue: f32 = 360.0 / (n as f32) * i as f32 - 180.0;


            let hsv: palette::Hsv = palette::Hsv::from_components((hue, 1.0, 1.0));

            let (r, g, b) = palette::Srgb::from(hsv).into_components();

            canvas
                .circle(
                    prevx as i16,
                    prevy as i16,
                    radius as i16,
                    Color::RGB((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8),
                )
                .unwrap();
        }

        canvas.circle(x as i16, y as i16, 1, Color::RGB(0, 0, 0)).unwrap();

        canvas.set_draw_color(Color::RGB(128, 128, 128));
        canvas.draw_line(
            sdl2::rect::Point::new(x as i32, y as i32),
            sdl2::rect::Point::new(400, y as i32),
        ).unwrap();

        if !pause || tempadd {
            points.insert(0, y as i16);
            tempadd = false;
        }

        if points.len() > 700 {
            points.remove(points.len() - 1);
        }

        for (i, p) in points.iter().enumerate() {
            if i != 0 {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                canvas
                    .draw_line(
                        sdl2::rect::Point::new((i + 399) as i32, points[i - 1] as i32),
                        sdl2::rect::Point::new((i + 400) as i32, *p as i32),
                    )
                    .unwrap();
            }
        }

        if !pause {
            angle += 0.01;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 240));
    }
}
