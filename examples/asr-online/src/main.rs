use sherpa_rs_next::{
    asr::online::{AsrOnlineRecognizer, paraformer::ParaformerAsrOnlineConfig},
    audio::read_audio_file,
};

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).expect("Missing file path argument");
    let encoder = std::env::args()
        .nth(2)
        .expect("Missing model path argument");
    let decoder = std::env::args()
        .nth(3)
        .expect("Missing tokens path argument");
    let tokens = std::env::args()
        .nth(4)
        .expect("Missing tokens path argument");

    let mut config = ParaformerAsrOnlineConfig::default();
    config
        .with_encoder(&encoder)
        .with_decoder(&decoder)
        .with_model_tokens(&tokens)
        .with_enable_endpoint(true)
        .with_rule1_min_trailing_silence(2.4)
        .with_rule2_min_trailing_silence(1.2)
        .with_rule3_min_utterance_length(300.0);

    let (samples, sample_rate) = read_audio_file(&file)?;
    let mut recognizer = AsrOnlineRecognizer::create(config)?;

    for chunk in samples.chunks(1600) {
        let result = recognizer.transcribe(sample_rate, &chunk)?;
        if result.text().is_empty() {
            continue;
        }
        if result.is_final() {
            println!("🎉 Final: {}", result.text());
        } else {
            println!("💬 Partial: {}", result.text());
        }
    }

    Ok(())
}
