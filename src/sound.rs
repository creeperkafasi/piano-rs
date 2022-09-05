use std::thread;
use std::time::Duration;

use rodio::{OutputStream, Sink};

// For testing, plays the first few notes in Ode to Joy by Beethoven
fn _play_music() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let notes = ["E5", "E5", "F5", "G5", "G5", "F5", "E5", "D5"];

    for note in notes {
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(piano::PianoWave::new(piano::KEYS[note]));
        thread::sleep(Duration::from_millis(600));
    }
}

pub fn play_note(key: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(piano::PianoWave::new(piano::KEYS[key]));
    thread::sleep(Duration::from_millis(2000));
}

pub mod piano {
    use std::{collections::HashMap, f32::consts::PI, time::Duration};

    use rodio::Source;

    lazy_static! {
        pub static ref KEYS: HashMap<&'static str, f32> = HashMap::from([
            ("C5", 523.25),
            ("D5", 587.33),
            ("E5", 659.25),
            ("F5", 698.46),
            ("G5", 783.99),
            ("A5", 880.00),
            ("B5", 987.77),
            ("", 0.0),
        ]);
    }

    /// An infinite source that produces a sine.
    ///
    /// Always has a rate of 48kHz and one channel.
    #[derive(Clone, Debug)]
    pub struct PianoWave {
        freq: f32,
        num_sample: usize,
    }

    impl PianoWave {
        // const Keys : HashMap<%str, f32> = HashMap::from!([
        //     ("", 100.0),
        // ]);

        /// The frequency of the sine.
        #[inline]
        pub fn new(freq: f32) -> PianoWave {
            PianoWave {
                freq: freq,
                num_sample: 0,
            }
        }

        pub fn wave_function(w: f32, t: f32, overtones: i32) -> f32 {
            let mut y = 0.0;
            // Base sine
            y += (w * t).sin() * (w * t * -0.0004).exp();
            // Overtones
            for n in 0..overtones {
                y += (w * t * 2f32.powi(n)).sin() * (w * t * -0.0004).exp() / 2f32.powi(n);
            }
            // y += y*y;

            // y += (t * w).sin().powi(3) + (w * (t + (2. / 3.))).sin();

            return y / 2.2; //TODO: FIGURE OUT WHY I NEED TO DIVIDE BY THIS VALUE
        }
    }

    impl Iterator for PianoWave {
        type Item = f32;

        #[inline]
        fn next(&mut self) -> Option<f32> {
            self.num_sample = self.num_sample.wrapping_add(1);

            let t = self.num_sample as f32 / 48000.0;
            let w = 2.0 * PI * self.freq as f32;

            // let y = (2.0 * PI * self.freq * t).sin() * f32::exp(t * -4.0) ;

            // y *= 1. + 16.0 * t * (-6.0 * t).exp();

            Some(PianoWave::wave_function(w, t, 10) as f32)
        }
    }

    impl Source for PianoWave {
        #[inline]
        fn current_frame_len(&self) -> Option<usize> {
            None
        }

        #[inline]
        fn channels(&self) -> u16 {
            1
        }

        #[inline]
        fn sample_rate(&self) -> u32 {
            48000
        }

        #[inline]
        fn total_duration(&self) -> Option<Duration> {
            None
        }
    }
}
