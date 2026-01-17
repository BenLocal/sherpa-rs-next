# sherpa-rs-next

A Rust wrapper for [sherpa-rs](https://github.com/k2-fsa/sherpa-rs), providing high-level APIs for speech recognition, text-to-speech, and voice activity detection.

## Features

- **ASR (Automatic Speech Recognition)**
  - Offline recognition with Paraformer and SenseVoice models
  - Online/streaming recognition with Zipformer and Zipformer-CTC models
  
- **TTS (Text-to-Speech)**
  - Offline TTS with Kokoro model
  
- **VAD (Voice Activity Detection)**
  - Silero VAD
  - Ten VAD

- **Audio Utilities**
  - Audio file reading and writing

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sherpa-rs-next = { git = "https://github.com/BenLocal/sherpa-rs-next.git" }
```

## Examples

### Offline ASR

```rust
use sherpa_rs_next::{
    asr::offline::{AsrOfflineConfig, AsrOfflineRecognizer, paraformer::ParaformerAsrOfflineConfig},
    audio::read_audio_file,
};

let mut config = ParaformerAsrOfflineConfig::default();
config
    .with_model("path/to/model.onnx")
    .with_model_tokens("path/to/tokens.txt");

let (samples, sample_rate) = read_audio_file("audio.wav")?;
let recognizer = AsrOfflineRecognizer::create_with_config(Box::new(config))?;
let result = recognizer.transcribe(sample_rate, &samples)?;
println!("Result: {}", result.text());
```

### Offline TTS

```rust
use sherpa_rs_next::tts::offline::{OfflineTTS, kokoro::KokoroTTSConfig};

let mut config = KokoroTTSConfig::default();
config
    .with_model("path/to/model.onnx")
    .with_voices("path/to/voices.bin")
    .with_tokens("path/to/tokens.txt");

let tts = OfflineTTS::create(&config)?;
let audio = tts.generate("Hello, world!", 0, 1.0)?;
```

## Running Examples

The project includes several examples in the `examples/` directory:

```bash
# Offline ASR
cargo run --example asr-offline -- paraformer audio.wav model.onnx tokens.txt

# Online ASR
cargo run --example asr-online

# Offline TTS
cargo run --example tts-offline -- /path/to/model/directory
```

## License

This project is based on [sherpa-rs](https://github.com/k2-fsa/sherpa-rs). Please refer to the original project for license information.
