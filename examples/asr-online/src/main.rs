use sherpa_rs_next::{
    asr::online::{
        AsrOnlineConfig, AsrOnlineRecognizer, paraformer::ParaformerAsrOnlineConfig,
        zipformer::ZipformerAsrOnlineConfig, zipformer_ctc::ZipformerCtcAsrOnlineConfig,
    },
    audio::read_audio_file,
};

fn main() -> anyhow::Result<()> {
    let model_type = std::env::args()
        .nth(1)
        .expect("Missing model type argument");
    let file = std::env::args().nth(2).expect("Missing file path argument");

    let config = match model_type.as_str() {
        "paraformer" => paraformer(),
        "zipformer_ctc" => zipformer_ctc(),
        "zipformer" => zipformer(),
        _ => anyhow::bail!("Invalid model type"),
    };

    let (samples, sample_rate) = read_audio_file(&file)?;
    let mut recognizer = AsrOnlineRecognizer::create_with_config(config)?;

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

/// https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/sherpa-onnx-streaming-paraformer-bilingual-zh-en.tar.bz2
fn paraformer() -> AsrOnlineConfig {
    let encoder = std::env::args()
        .nth(3)
        .expect("Missing model path argument");
    let decoder = std::env::args()
        .nth(4)
        .expect("Missing tokens path argument");
    let tokens = std::env::args()
        .nth(5)
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

    Box::new(config)
}

/// https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/sherpa-onnx-streaming-zipformer-ctc-multi-zh-hans-2023-12-13.tar.bz2
/// zipformer ctc multi-language ASR model for Chinese (Mandarin) and English
fn zipformer_ctc() -> AsrOnlineConfig {
    let model = std::env::args()
        .nth(3)
        .expect("Missing model path argument");
    let tokens = std::env::args()
        .nth(4)
        .expect("Missing tokens path argument");
    let mut config = ZipformerCtcAsrOnlineConfig::default();
    config
        .with_model(&model)
        .with_model_tokens(&tokens)
        .with_enable_endpoint(true)
        .with_rule1_min_trailing_silence(2.4)
        .with_rule2_min_trailing_silence(1.2)
        .with_rule3_min_utterance_length(300.0);

    Box::new(config)
}

/// https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/sherpa-onnx-streaming-zipformer-bilingual-zh-en-2023-02-20.tar.bz2
fn zipformer() -> AsrOnlineConfig {
    let encoder = std::env::args()
        .nth(3)
        .expect("Missing model path argument");
    let decoder = std::env::args()
        .nth(4)
        .expect("Missing model path argument");
    let joiner = std::env::args()
        .nth(5)
        .expect("Missing model path argument");
    let tokens = std::env::args()
        .nth(6)
        .expect("Missing tokens path argument");
    let mut config = ZipformerAsrOnlineConfig::default();
    config
        .with_encoder(&encoder)
        .with_decoder(&decoder)
        .with_joiner(&joiner)
        .with_model_tokens(&tokens)
        .with_enable_endpoint(true)
        .with_decoding_method("greedy_search");

    Box::new(config)
}
