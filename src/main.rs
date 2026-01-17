use std::fs::File;

use wayland_client::Connection;

use crate::window::State;

mod window;
mod shapes;
mod types;

fn main() {
    let conn = Connection::connect_to_env().unwrap();

    let mut event_queue = conn.new_event_queue();
    let qhandle = event_queue.handle();

    let display = conn.display();
    display.get_registry(&qhandle, ());

    let mut state = State {
        running: true,
        base_surface: None,
        buffer: None,
        wm_base: None,
        xdg_surface: None,
        configured: false,
        draw: None,
    };


    println!("Starting the example window app, press <ESC> to quit.");

    state.draw = Some(draw_checker_board);

    while state.running {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }
}


fn draw_checker_board(tmp: &mut File, (buf_x, buf_y): (isize, isize)) {
    use std::io::Write;

    println!("Selected Draw Function Checker Board");

    let mut buf = std::io::BufWriter::new(tmp);
    for y in 0..buf_y {
        for x in 0..buf_x {
            if (x / 25) % 2 == (y / 25) % 2 {
                buf.write_all(&[0x00, 0x00, 0xFF, 0xFF]).unwrap();
            } else {
                buf.write_all(&[0x00, 0x00, 0x00, 0xFF]).unwrap();
            }
        }
    }
    buf.flush().unwrap();
}
