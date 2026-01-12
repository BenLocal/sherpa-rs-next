use std::path::Path;

use sherpa_rs_next::{
    audio::write_audio_file,
    tts::offline::{OfflineTTS, kokoro::KokoroTTSConfig},
};

fn main() -> anyhow::Result<()> {
    let path = std::env::args()
        .nth(1)
        .expect("Missing model path argument");

    let model = Path::new(&path).join("model.onnx");
    let voices = Path::new(&path).join("voices.bin");
    let tokens = Path::new(&path).join("tokens.txt");
    let data_dir = Path::new(&path).join("espeak-ng-data");
    let dict_dir = Path::new(&path).join("dict");
    let en_lexicon = Path::new(&path).join("lexicon-us-en.txt");
    let zh_lexicon = Path::new(&path).join("lexicon-zh.txt");

    let mut config = KokoroTTSConfig::default();
    config
        .with_model(model.to_str().unwrap())
        .with_voices(voices.to_str().unwrap())
        .with_tokens(tokens.to_str().unwrap())
        .with_data_dir(data_dir.to_str().unwrap())
        .with_dict_dir(dict_dir.to_str().unwrap())
        .with_lexicon_files(&[en_lexicon.to_str().unwrap(), zh_lexicon.to_str().unwrap()]);

    let tts = OfflineTTS::create(&config)?;

    let audio = tts.generate("Hello, world!, 你好，世界！", 0, 1.0)?;
    println!("Audio duration: {} seconds", audio.duration());
    println!("Audio sample rate: {} Hz", audio.sample_rate());
    println!("Audio samples: {}", audio.samples().len());

    let audio_file = Path::new("output.wav");
    write_audio_file(
        audio_file.to_str().unwrap(),
        audio.samples(),
        audio.sample_rate() as u32,
    )?;

    Ok(())
}
