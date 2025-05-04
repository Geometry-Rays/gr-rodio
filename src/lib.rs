use rodio::OutputStreamHandle;
use std::fs::File;
use std::io::BufReader;

#[allow(unused_imports)]
use rodio::{Decoder, OutputStream, source::Source};

pub fn play_audio(
    path: &str,
    stream_handle: OutputStreamHandle
) {
    let menu_loop_sound: Decoder<BufReader<File>> = Decoder::new(
        BufReader::new(
            File::open(path).unwrap()
        )
    ).unwrap();

    stream_handle.play_raw(menu_loop_sound.convert_samples()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_testing() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        play_audio("./menu-music.ogg", stream_handle);

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
