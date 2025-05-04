use std::fs::File;
use std::io::BufReader;

#[allow(unused_imports)]
use rodio::{Decoder, OutputStream};

pub fn play_audio(
    path: &str,
    volume: f32,
    sink: &rodio::Sink
) {
    let menu_loop_sound: Decoder<BufReader<File>> = Decoder::new(
        BufReader::new(
            File::open(path).unwrap()
        )
    ).unwrap();

    sink.append(menu_loop_sound);
    sink.play();
    sink.set_volume(volume);
}

pub fn stop_audio(
    sink: &rodio::Sink
) {
    sink.stop();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_testing() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();

        play_audio("./menu-music.ogg", 2.0, &sink);

        std::thread::sleep(std::time::Duration::from_secs(1));

        stop_audio(&sink);

        std::thread::sleep(std::time::Duration::from_secs(1));

        play_audio("./menu-music.ogg", 2.0, &sink);

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}