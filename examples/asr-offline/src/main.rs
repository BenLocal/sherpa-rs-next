use sherpa_rs_next::asr::offline::{
    AsrOfflineConfig, AsrOfflineRecognizer, paraformer::ParaformerAsrOfflineConfig,
    sense_voice::SenseVoiceAsrOfflineConfig,
};

fn main() -> anyhow::Result<()> {
    let model_type = std::env::args()
        .nth(1)
        .expect("Missing model type argument");
    let file = std::env::args().nth(2).expect("Missing file path argument");
    let model = std::env::args()
        .nth(3)
        .expect("Missing model path argument");
    let tokens = std::env::args()
        .nth(4)
        .expect("Missing tokens path argument");

    let config: AsrOfflineConfig = match model_type.as_str() {
        "paraformer" => {
            let mut config = ParaformerAsrOfflineConfig::default();
            config
                .with_model(&model)
                .with_model_tokens(&tokens)
                .with_model_debug(true);
            Box::new(config)
        }
        "sense_voice" => {
            let mut config = SenseVoiceAsrOfflineConfig::default();
            config
                .with_model(&model)
                .with_model_tokens(&tokens)
                .with_language("en")
                .with_use_itn(true)
                .with_model_debug(true);
            Box::new(config)
        }
        _ => anyhow::bail!("Invalid model type"),
    };

    let (samples, sample_rate) = read_audio_file(&file)?;
    let recognizer = AsrOfflineRecognizer::create_with_config(config)?;

    let result = recognizer.transcribe(sample_rate, &samples)?;
    println!("Result: {}", result.text());

    Ok(())
}

pub fn read_audio_file(path: &str) -> anyhow::Result<(Vec<f32>, u32)> {
    let mut reader = hound::WavReader::open(path)?;
    let sample_rate = reader.spec().sample_rate;

    // Check if the sample rate is 16000
    if sample_rate != 16000 {
        anyhow::bail!("The sample rate must be 16000.");
    }

    // Collect samples into a Vec<f32>
    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| (s.unwrap() as f32) / (i16::MAX as f32))
        .collect();

    Ok((samples, sample_rate))
}
