use std::fs::File;
use std::io::BufReader;

#[allow(unused_imports)]
use rodio::{Decoder, OutputStream};

pub fn play_audio_path(
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

pub fn play_audio(
    audio: Decoder<BufReader<File>>,
    volume: f32,
    sink: &rodio::Sink
) {
    sink.append(audio);
    sink.play();
    sink.set_volume(volume);
}

pub fn load_audio(
    path: &str
) -> Decoder<BufReader<File>> {
    return Decoder::new(
        BufReader::new(
            File::open(path).unwrap()
        )
    ).unwrap()
}

pub fn stop_audio(
    sink: &rodio::Sink
) {
    sink.stop();
}

// This can only be used if your using an mp3 or a wav
// If your using an ogg then it will panic
pub fn restart_audio(
    sink: &rodio::Sink
) {
    sink.try_seek(std::time::Duration::from_secs(0)).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_testing() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();

        let test_sound = load_audio("./menu-music.mp3");

        play_audio_path("./menu-music.mp3", 2.0, &sink);

        std::thread::sleep(std::time::Duration::from_secs(1));

        stop_audio(&sink);

        std::thread::sleep(std::time::Duration::from_secs(1));

        play_audio(test_sound, 2.0, &sink);

        std::thread::sleep(std::time::Duration::from_secs(1));

        restart_audio(&sink);

        std::thread::sleep(std::time::Duration::from_secs_f32(0.5));

        restart_audio(&sink);

        std::thread::sleep(std::time::Duration::from_secs_f32(0.5));

        restart_audio(&sink);

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

// This is so people can import stuff from rodio
// This wrapper isn't a full wrapper so thats why this is here
pub mod rodio_raw {
    pub use rodio::*;
}