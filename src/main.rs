pub mod convert;
pub mod piano;
pub mod viz;

use opencv::highgui::{named_window, imshow, wait_key, WINDOW_OPENGL};
use piano::{Piano, Piano2D, Piano3D, PianoBorder};

fn main() {
    let piano2d = Piano2D::new(1920.0, 240.0, 0, 88);
    let border = PianoBorder::new(
        (625.0, 767.0),
        (1662.0, 242.0),
        (1824.0, 267.0),
        (835.0, 905.0),
    );
    let mut piano = Piano3D::new(border, piano2d);

    named_window("frame", WINDOW_OPENGL).unwrap();

    let mut frame = viz::frame(1920, 1080, (0.0, 255.0, 0.0)).unwrap();

    for key in 0..88 {
        if piano.key_is_black(key) {
            continue;
        }
        viz::fill_key(&mut frame, &mut piano, key, (128.0, 128.0, 128.0)).unwrap();
        viz::outline_key(&mut frame, &mut piano, key, (255.0, 255.0, 255.0)).unwrap();
    }

    for key in 0..88 {
        if !piano.key_is_black(key) {
            continue;
        }
        viz::fill_key(&mut frame, &mut piano, key, (64.0, 64.0, 64.0)).unwrap();
        viz::outline_key(&mut frame, &mut piano, key, (255.0, 255.0, 255.0)).unwrap();
    }

    imshow("frame", &frame).unwrap();

    wait_key(10000).unwrap();
}
