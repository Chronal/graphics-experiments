extern crate sdl2;

use std::convert::TryInto;

use sdl2::event::Event;
use sdl2::event::EventType;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    /*
    let controller_subsystem = sdl_context.game_controller().unwrap();
    println!("{:#?}", controller_subsystem.num_joysticks());
    let controller = controller_subsystem.open(0).unwrap();
    */

    let window_height = 800;
    let window_width = 600;

    let black = Color::RGB(0,0,0);
    let red = Color::RGB(255,0,0);
    let blue = Color::RGB(0,0,255);

    let window = video_subsystem.window("rust-sdl2 demo", window_height, window_width)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let origin_rect = Rect::new((3*window_height/8).try_into().unwrap(), (window_width/2).try_into().unwrap(),200,200);
    let mut rect = Rect::new(0,0, 100, 100);

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.fill_rect(origin_rect);

    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    println!("{:#?}", event_pump.is_event_enabled(EventType::ControllerButtonDown));

    'running: loop {

        canvas.set_draw_color(red);
        canvas.clear();
        canvas.set_draw_color(black);
        canvas.fill_rect(origin_rect);

        for event in event_pump.poll_iter() {
            println!("{:#?}",event);
            match event {
                Event::Quit {..} | Event::KeyDown { keycode : Some(Keycode::Escape), .. } => {
                    break 'running
                },
                
                Event::KeyDown { keycode : Some(Keycode::Right), .. } | Event::KeyDown { keycode : Some(Keycode::D), .. } => {
                    rect.x += 30; 
                },

                Event::KeyDown { keycode : Some(Keycode::Left), .. } | Event::KeyDown { keycode : Some(Keycode::A), .. } => {
                    rect.x -= 30; 
                },

                Event::KeyDown { keycode : Some(Keycode::Down), .. } | Event::KeyDown { keycode : Some(Keycode::S), .. } => {
                    rect.y += 30; 
                },

                Event::KeyDown { keycode : Some(Keycode::Up), .. } | Event::KeyDown { keycode : Some(Keycode::W), .. } => {
                    rect.y -= 30; 
                },
                _ => {}
            }
        }

        canvas.set_draw_color(blue);
        canvas.fill_rect(rect);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32/60));
    }
}
