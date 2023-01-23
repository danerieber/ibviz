pub mod convert;
pub mod piano;
pub mod viz;

use std::{fs, time};

use midly::{MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};
use opencv::highgui::{imshow, named_window, wait_key, WINDOW_OPENGL};
use piano::{IbRectangle, Piano, Piano2D, Piano3D};

fn main() {
    let piano2d = Piano2D::new(1920.0, 240.0, 0, 88);
    let border = IbRectangle::new(
        (625.0, 767.0),
        (1662.0, 242.0),
        (1824.0, 267.0),
        (835.0, 905.0),
    );
    let mut piano = Piano3D::new(border, piano2d);

    named_window("frame", WINDOW_OPENGL).unwrap();

    let mut frame = viz::frame(1920, 1080, (0.0, 255.0, 0.0)).unwrap();

    for key in 0..88 {
        if piano.is_black(key) {
            continue;
        }
        viz::fill_key(&mut frame, &mut piano, key, (128.0, 128.0, 128.0)).unwrap();
        viz::outline_key(&mut frame, &mut piano, key, (255.0, 255.0, 255.0)).unwrap();
    }

    for key in 0..88 {
        if !piano.is_black(key) {
            continue;
        }
        viz::fill_key(&mut frame, &mut piano, key, (64.0, 64.0, 64.0)).unwrap();
        viz::outline_key(&mut frame, &mut piano, key, (255.0, 255.0, 255.0)).unwrap();
    }

    let mid_data = fs::read("no4.mid").unwrap();
    let smf = Smf::parse(&mid_data).unwrap();
    let mut ticks_per_beat: midly::num::u15 = 48.into();
    if let Timing::Metrical(tpb) = smf.header.timing {
        ticks_per_beat = tpb;
    }
    let track = smf.tracks.first().unwrap();

    let mut notes_on: Vec<(midly::num::u7, midly::num::u7)> = vec![];

    let mut tempo: midly::num::u24 = 500000.into();
    for event in track {
        let mut new_frame = frame.clone();
        for note in notes_on.iter() {
            viz::fill_key_scale_height(
                &mut new_frame,
                &mut piano,
                note.0.as_int() as usize - 21,
                1.5,
                (0.0, 0.0, 255.0),
            )
            .unwrap();
        }
        imshow("result", &new_frame).unwrap();
        let delta = if event.delta.as_int() > 0 {
            tempo.as_int() * event.delta.as_int() / ticks_per_beat.as_int() as u32
        } else {
            0
        };
        let delay = time::Duration::from_micros(delta as u64).as_millis() as i32;
        if delay > 0 {
            wait_key(delay).unwrap();
        }
        match event.kind {
            TrackEventKind::Meta(message) => {
                if let MetaMessage::Tempo(value) = message {
                    tempo = value;
                }
            }
            TrackEventKind::Midi { channel: _, message } => {
                if let MidiMessage::NoteOn { key, vel } = message {
                    notes_on.push((key, vel));
                } else if let MidiMessage::NoteOff { key, vel: _ } = message {
                    notes_on = notes_on.into_iter().filter(|&note| note.0 != key).collect();
                }
            }
            TrackEventKind::SysEx(_) => {}
            TrackEventKind::Escape(_) => {}
        }
    }

    wait_key(10000).unwrap();
}
