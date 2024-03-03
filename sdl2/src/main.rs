use nove_core::cartridge::Rom;
use nove_core::core::NesNoveCore;
use nove_core::memory::Memory;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::EventPump;
use std::time::Duration;
use structopt::StructOpt;

const WIDTH: u32 = 32;
const HEIGHT: u32 = 32;
const RGB_SPACE: u32 = 3;
const SCALE: u32 = 10;

const RAND_ADDR: u16 = 0xfe;
const KEY_ADDR: u16 = 0xff;

#[derive(Debug, StructOpt)]
pub struct Args {
    /// The ROM file to read
    pub file: String,
}

fn color(byte: u8) -> Color {
    match byte {
        0x0 => Color::BLACK,
        0x1 => Color::WHITE,
        0x2 => Color::RED,
        0x3 => Color::CYAN,
        0x4 => Color::MAGENTA,
        0x5 => Color::GREEN,
        0x6 => Color::BLUE,
        0x7 => Color::YELLOW,
        0x8 => (255, 128, 0).into(),   // orange
        0x9 => (128, 64, 0).into(),    // brown
        0xa => (255, 64, 64).into(),   // light red
        0xb => (192, 192, 192).into(), // light grey
        0xc => Color::GREY,
        0xd => (64, 255, 64).into(), // light green
        0xe => (64, 64, 255).into(), // light blue
        0xf => (64, 64, 64).into(),  // dark grey
        _ => unimplemented!("only 16 colors are supported"),
    }
}

fn read_screen_state(
    cpu: &NesNoveCore,
    frame: &mut [u8; (WIDTH * HEIGHT * RGB_SPACE) as usize],
) -> bool {
    let mut frame_idx = 0;
    let mut update = false;
    for i in 0x0200..0x600 {
        let color_idx = cpu.memory.read(i as u16);
        let (b1, b2, b3) = color(color_idx).rgb();
        if frame[frame_idx] != b1 || frame[frame_idx + 1] != b2 || frame[frame_idx + 2] != b3 {
            frame[frame_idx] = b1;
            frame[frame_idx + 1] = b2;
            frame[frame_idx + 2] = b3;
            update = true;
        }
        frame_idx += 3;
    }
    update
}

fn handle_user_input(cpu: &mut NesNoveCore, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                cpu.memory.write(KEY_ADDR, b'w');
            }
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                cpu.memory.write(KEY_ADDR, b's');
            }
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                cpu.memory.write(KEY_ADDR, b'a');
            }
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                cpu.memory.write(KEY_ADDR, b'd');
            }
            _ => { /* do nothing */ }
        }
    }
}

fn main() {
    let args = Args::from_args();

    let sdl_context = sdl2::init().expect("failed to load sdl2");

    let mut canvas = sdl_context
        .video()
        .expect("failed to load video subsystem")
        .window("Snake demo", WIDTH * SCALE, HEIGHT * SCALE)
        .position_centered()
        .build()
        .expect("failed to create rendering window")
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();
    canvas.set_scale(SCALE as f32, SCALE as f32).unwrap();

    let mut event_pump = sdl_context
        .event_pump()
        .expect("failed to create event pump");

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, WIDTH, HEIGHT)
        .unwrap();

    let content = std::fs::read(args.file).expect("failed to read rom file");
    let rom = Rom::new(&content).unwrap();

    let mut cpu = NesNoveCore::new(rom);
    cpu.reset();

    let mut screen_state = [0u8; (WIDTH * RGB_SPACE * HEIGHT) as usize];
    let mut rng = rand::thread_rng();

    cpu.run(move |cpu| {
        handle_user_input(cpu, &mut event_pump);

        cpu.memory.write(RAND_ADDR, rng.gen_range(1..16));

        if read_screen_state(cpu, &mut screen_state) {
            texture
                .update(None, &screen_state, (WIDTH * RGB_SPACE) as usize)
                .unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }

        std::thread::sleep(Duration::new(0, 70_000));
    })
    .unwrap();
}
