extern crate nove_core;
extern crate rand;
extern crate sdl2;

use nove_core::core::{Core6502, NoveCore};
use nove_core::interrupt::InterruptFlag;
use nove_core::memory::cpu_mem::CpuMem;
use nove_core::memory::Memory;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::EventPump;
use std::time::Duration;

const WIDTH: u32 = 32;
const HEIGHT: u32 = 32;
const RGB_SPACE: u32 = 3;
const SCALE: u32 = 10;

const RAND_ADDR: u16 = 0xfe;
const KEY_ADDR: u16 = 0xff;

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
    cpu: &NoveCore<CpuMem>,
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

fn handle_user_input(cpu: &mut NoveCore<CpuMem>, event_pump: &mut EventPump) {
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

    // https://gist.github.com/wkjagt/9043907
    let game_code = vec![
        //
        0x20, 0x06, 0x06, // jsr init
        0x20, 0x38, 0x06, // jsr loop
        // init:
        0x20, 0x0d, 0x06, // jsr init_snake
        0x20, 0x2a, 0x06, // jsr generate_apple_position
        0x60, // rts (:2)
        // init_snake:
        0xa9, 0x02, // lda #$02
        0x85, 0x02, // sta $02
        0xa9, 0x04, // lda #$04
        0x85, 0x03, // sta $03
        0xa9, 0x11, // lda #$11
        0x85, 0x10, // sta $10
        0xa9, 0x10, // lda #$10
        0x85, 0x12, // sta $12
        0xa9, 0x0f, // lda #$0f
        0x85, 0x14, // sta $14
        0xa9, 0x04, // lda #$04
        0x85, 0x11, // sta $11
        0x85, 0x13, // sta $13
        0x85, 0x15, // sta $15
        0x60, // rts (init:2)
        // generate_apple_position:
        0xa5, 0xfe, // lda $fe
        0x85, 0x00, // sta $00
        0xa5, 0xfe, // lda $fe
        0x29, 0x03, // and #$03
        0x18, // clc
        0x69, 0x02, // adc #$02
        0x85, 0x01, // sta $01
        0x60, // rts (init:3)
        // loop:
        0x20, 0x4d, 0x06, // jsr read_keys
        0x20, 0x8d, 0x06, // jsr check_collision
        0x20, 0xc3, 0x06, // jsr update_snake
        0x20, 0x19, 0x07, // jsr draw_apple
        0x20, 0x20, 0x07, // jsr draw_snake
        0x20, 0x2d, 0x07, // jsr spin_wheels
        0x4c, 0x38, 0x06, // jmp loop
        // read_keys:
        0xa5, 0xff, // lda $ff
        0xc9, 0x77, // cmp #$77 (W)
        0xf0, 0x0d, // beq up_key
        0xc9, 0x64, // cmp #$64 (D)
        0xf0, 0x14, // beq right_key
        0xc9, 0x73, // cmp #$73 (S)
        0xf0, 0x1b, // beq down_key
        0xc9, 0x61, // cmp #$61 (A)
        0xf0, 0x22, // beq left_key
        0x60, // rts (loop:2)
        // up_key:
        0xa9, 0x04, // lda #$04
        0x24, 0x02, // bit $02
        0xd0, 0x26, // bne illegal_move
        0xa9, 0x01, // lda #$01
        0x85, 0x02, // sta $02
        0x60, // rts (loop:2)
        // right_key:
        0xa9, 0x08, // lda #$08
        0x24, 0x02, // bit $02
        0xd0, 0x1b, // bne illegal_move
        0xa9, 0x02, // lda #$02
        0x85, 0x02, // sta $02
        0x60, // rts (loop:2)
        // down_key:
        0xa9, 0x01, // lda #$01
        0x24, 0x02, // bit $02
        0xd0, 0x10, // bne illegal_move
        0xa9, 0x04, // lda #$04
        0x85, 0x02, // sta $02
        0x60, // rts (loop:2)
        // left_key:
        0xa9, 0x02, // lda #$02
        0x24, 0x02, // bit $02
        0xd0, 0x05, // bne illegal_move
        0xa9, 0x08, // lda #$08
        0x85, 0x02, // sta $02
        0x60, // rts (loop:2)
        // illegal_move:
        0x60, // rts (loop:2)
        // check_collision:
        0x20, 0x94, 0x06, // jsr check_apple_collision
        0x20, 0xa8, 0x06, // jsr check_snake_collision
        0x60, // rts (loop:3)
        // check_apple_collision:
        0xa5, 0x00, // lda $00
        0xc5, 0x10, // cmp $10
        0xd0, 0x0d, // bne apple_collision_checked
        0xa5, 0x01, // lda $01
        0xc5, 0x11, // cmp $11
        0xd0, 0x07, // bne apple_collision_checked
        0xe6, 0x03, // inc $03
        0xe6, 0x03, // inc $03
        0x20, 0x2a, 0x06, // jsr generate_apple
        // apple_collision_checked:
        0x60, // rts (check_collision:2)
        // check_snake_collision:
        0xa2, 0x02, // ldx #$02
        // snake_collision_loop:
        0xb5, 0x10, // lda $10,x
        0xc5, 0x10, // cmp $10
        0xd0, 0x06, // bne continue_collision_loop
        // maybe_snake_collision:
        0xb5, 0x11, // lda $11,x
        0xc5, 0x11, // cmp $11
        0xf0, 0x09, // beq snake_collision
        // continue_collision_loop:
        0xe8, // inx
        0xe8, // inx
        0xe4, 0x03, // cpx $03
        0xf0, 0x06, // beq no_snake_collision
        0x4c, 0xaa, 0x06, // jmp snake_collision_loop
        // snake_collision:
        0x4c, 0x35, 0x07, // jmp game_over
        // no_snake_collision:
        0x60, // rts (check_collision:3)
        // update_snake:
        0xa6, 0x03, // ldx $03
        0xca, // dex
        0x8a, // txa
        // update_loop:
        0xb5, 0x10, // lda $10,x
        0x95, 0x12, // sta $12,x
        0xca, // dex
        0x10, 0xf9, // bpl update_loop
        0xa5, 0x02, // lda $02
        0x4a, // lsr
        0xb0, 0x09, // bcs up
        0x4a, // lsr
        0xb0, 0x19, // bcs right
        0x4a, // lsr
        0xb0, 0x1f, // bcs down
        0x4a, // lsr
        0xb0, 0x2f, // bcs left
        // up:
        0xa5, 0x10, // lda $10
        0x38, // sec
        0xe9, 0x20, // sbc #$20
        0x85, 0x10, // sta $10
        0x90, 0x01, // bcc up_up
        0x60, // rts (loop:4)
        // up_up:
        0xc6, 0x11, // dec $11
        0xa9, 0x01, // lda #$01
        0xc5, 0x11, // cmp $11
        0xf0, 0x28, // beq wall_collision
        0x60, // rts (loop:4)
        // right:
        0xe6, 0x10, // inc $10
        0xa9, 0x1f, // lda #$1f
        0x24, 0x10, // bit $10
        0xf0, 0x1f, // beq wall_collision
        0x60, // rts (loop:4)
        // down:
        0xa5, 0x10, // lda $10
        0x18, // clc
        0x69, 0x20, // adc #$20
        0x85, 0x10, // sta $10
        0xb0, 0x01, // bcs down_down
        0x60, // rts (loop:4)
        // down_down:
        0xe6, 0x11, // inc $11
        0xa9, 0x06, // lda #$06
        0xc5, 0x11, // cmp $11
        0xf0, 0x0c, // beq wall_collision
        0x60, // rts (loop:4)
        // left:
        0xc6, 0x10, // dec $10
        0xa5, 0x10, // lda $10
        0x29, 0x1f, // and #$1f
        0xc9, 0x1f, // cmp #$1f
        0xf0, 0x01, // beq wall_collision
        0x60, // rts (loop:4)
        // wall_collision:
        0x4c, 0x35, 0x07, // jmp game_over
        // draw_apple:
        0xa0, 0x00, // ldy #$00
        0xa5, 0xfe, // lda $fe
        0x91, 0x00, // sta ($00),y
        0x60, // rts (loop:5)
        // draw_snake:
        0xa2, 0x00, // ldx #$00
        0xa9, 0x01, // lda #$01
        0x81, 0x10, // sta ($10),x
        0xa6, 0x03, // ldx $03
        0xa9, 0x00, // lda #$00
        0x81, 0x10, // sta ($10),x
        0x60, // rts (loop:6)
        // spin_wheels:
        0xa6, 0xff, // ldx #$00
        // spin_loop:
        0xea, // nop
        0xea, // nop
        0xca, // dex
        0xd0, 0xfb, // bne spin_loop
        0x60, // rts (loop:7)
              // game_over:
    ];

    let mut core = Core6502::new();
    core.snake_load(game_code);
    core.reset();

    let mut screen_state = [0u8; (WIDTH * RGB_SPACE * HEIGHT) as usize];
    let mut rng = rand::thread_rng();

    loop {
        if let InterruptFlag::BRK = core.tick().unwrap() {
            return;
        }

        handle_user_input(&mut core, &mut event_pump);

        core.memory.write(RAND_ADDR, rng.gen_range(1..16));

        if read_screen_state(&core, &mut screen_state) {
            texture
                .update(None, &screen_state, (WIDTH * RGB_SPACE) as usize)
                .unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }

        std::thread::sleep(Duration::new(0, 70_000));
    }
}
