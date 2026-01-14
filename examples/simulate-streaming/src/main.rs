use std::{sync::mpsc, time::Instant};

use colored::Colorize as _;
use cpal::traits::{DeviceTrait as _, HostTrait as _, StreamTrait as _};
use sherpa_rs_next::{
    asr::offline::{AsrOfflineRecognizer, sense_voice::SenseVoiceAsrOfflineConfig},
    vad::{Vad, silero::SileroVadConfig},
};

fn main() -> anyhow::Result<()> {
    let vad_model = std::env::args()
        .nth(1)
        .expect("Missing vad model type argument");
    let model = std::env::args()
        .nth(2)
        .expect("Missing model path argument");
    let tokens = std::env::args()
        .nth(3)
        .expect("Missing tokens path argument");

    let (stop_tx, stop_rx) = mpsc::channel();
    ctrlc::set_handler(move || {
        println!("\nCaught Ctrl + C. Exiting...");
        let _ = stop_tx.send(());
    })
    .map_err(|e| anyhow::anyhow!("Failed to set Ctrl + C handler: {}", e))?;

    let mut vad_config = SileroVadConfig::default();
    vad_config
        .with_debug(true)
        .with_model(&vad_model)
        .with_min_silence_duration(0.1)
        .with_min_speech_duration(0.25)
        .with_max_speech_duration(8.0)
        .with_window_size(512)
        .with_sample_rate(16000)
        .with_threshold(0.5);

    let mut vad = Vad::create(vad_config, 20.0)?;

    let mut recognizer_config = SenseVoiceAsrOfflineConfig::default();
    recognizer_config
        .with_model(&model)
        .with_model_tokens(&tokens)
        .with_language("auto")
        .with_use_itn(true)
        .with_model_debug(true);

    let recognizer = AsrOfflineRecognizer::create(recognizer_config)?;
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or("No input device available")
        .map_err(|e| anyhow::anyhow!("No input device available: {}", e))?;
    // Get device info
    let device_id = device.id()?;
    println!("Using input device: {:?}", device_id);

    // Get default input config
    let config = device.default_input_config()?;
    let mic_sample_rate = config.sample_rate() as f32;
    let target_sample_rate = 16000.0;
    let channels = config.channels() as usize;

    println!("Device sample rate: {} Hz", mic_sample_rate);
    println!("Target sample rate: {} Hz", target_sample_rate);
    println!("Channels: {}", channels);

    let (audio_tx, audio_rx) = mpsc::channel::<Vec<f32>>();
    let err_fn = |err| eprintln!("Error occurred on stream: {}", err);
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => {
            let channels = channels;
            device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    // Convert to mono by averaging channels
                    let mono_samples: Vec<f32> = if channels > 1 {
                        data.chunks(channels)
                            .map(|chunk| chunk.iter().sum::<f32>() / channels as f32)
                            .collect()
                    } else {
                        data.to_vec()
                    };
                    if audio_tx.send(mono_samples).is_err() {
                        // Channel closed, stop sending
                    }
                },
                err_fn,
                None,
            )?
        }
        cpal::SampleFormat::I16 => {
            let channels = channels;
            device.build_input_stream(
                &config.into(),
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    // Convert to f32 and then to mono
                    let f32_samples: Vec<f32> =
                        data.iter().map(|&s| s as f32 / i16::MAX as f32).collect();
                    let mono_samples: Vec<f32> = if channels > 1 {
                        f32_samples
                            .chunks(channels)
                            .map(|chunk| chunk.iter().sum::<f32>() / channels as f32)
                            .collect()
                    } else {
                        f32_samples
                    };
                    if audio_tx.send(mono_samples).is_err() {
                        // Channel closed, stop sending
                    }
                },
                err_fn,
                None,
            )?
        }
        cpal::SampleFormat::U16 => {
            let channels = channels;
            device.build_input_stream(
                &config.into(),
                move |data: &[u16], _: &cpal::InputCallbackInfo| {
                    // Convert to f32 and then to mono
                    let f32_samples: Vec<f32> = data
                        .iter()
                        .map(|&s| (s as f32 / u16::MAX as f32) * 2.0 - 1.0)
                        .collect();
                    let mono_samples: Vec<f32> = if channels > 1 {
                        f32_samples
                            .chunks(channels)
                            .map(|chunk| chunk.iter().sum::<f32>() / channels as f32)
                            .collect()
                    } else {
                        f32_samples
                    };
                    if audio_tx.send(mono_samples).is_err() {
                        // Channel closed, stop sending
                    }
                },
                err_fn,
                None,
            )?
        }
        _ => return Err(anyhow::anyhow!("Unsupported sample format")),
    };

    stream.play()?;
    println!("Started! Please speak\n");

    // Audio processing loop
    let window_size = 512;
    let mut buffer = Vec::new();
    let mut offset = 0;
    let mut speech_started = false;
    let mut speech_start_index = 0; // Index in buffer where speech started
    let mut started_time = Instant::now();
    let mut last_result = String::new();

    loop {
        // Check for stop signal
        if stop_rx.try_recv().is_ok() {
            break;
        }

        // Receive audio samples
        match audio_rx.try_recv() {
            Ok(samples) => {
                // Handle resampling if needed
                if (mic_sample_rate - target_sample_rate).abs() > 1.0 {
                    // Simple linear interpolation resampling
                    let ratio = target_sample_rate / mic_sample_rate;
                    let output_len = (samples.len() as f32 * ratio) as usize;
                    let mut resampled = Vec::with_capacity(output_len);

                    for i in 0..output_len {
                        let src_pos = i as f32 / ratio;
                        let src_idx = src_pos as usize;
                        let frac = src_pos - src_idx as f32;

                        if src_idx + 1 < samples.len() {
                            // Linear interpolation
                            let sample =
                                samples[src_idx] * (1.0 - frac) + samples[src_idx + 1] * frac;
                            resampled.push(sample);
                        } else if src_idx < samples.len() {
                            resampled.push(samples[src_idx]);
                        } else {
                            resampled.push(0.0);
                        }
                    }
                    buffer.extend_from_slice(&resampled);
                } else {
                    buffer.extend_from_slice(&samples);
                }
            }
            Err(mpsc::TryRecvError::Empty) => {
                // No data available, continue
                std::thread::sleep(std::time::Duration::from_millis(10));
                continue;
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                eprintln!("Audio channel disconnected");
                break;
            }
        }

        // Process audio in windows
        while offset + window_size <= buffer.len() {
            let window = &buffer[offset..offset + window_size];
            vad.accept_waveform(window.to_vec());

            if !speech_started && vad.is_speech() {
                speech_started = true;
                // Record the buffer index where speech started
                // We need to account for the offset that has been processed
                speech_start_index = offset;
                started_time = Instant::now();
            }

            offset += window_size;
        }

        // Keep buffer size manageable when not in speech
        if !speech_started {
            if buffer.len() > 10 * window_size {
                let keep_size = 10 * window_size;
                buffer = buffer[buffer.len() - keep_size..].to_vec();
                offset = 0;
            }
        }

        // Perform recognition if speech detected and enough time elapsed
        let elapsed = started_time.elapsed();
        if speech_started && elapsed.as_secs_f32() > 0.2 && !buffer.is_empty() {
            // Use only the audio from speech start to current
            let speech_audio = if speech_start_index < buffer.len() {
                &buffer[speech_start_index..]
            } else {
                &buffer
            };
            let result = recognizer.transcribe(target_sample_rate as u32, speech_audio)?;
            if !result.text().is_empty() && result.text() != last_result {
                // Display intermediate result in yellow
                print!("\r{}", colored::ColoredString::from(result.text()).yellow());
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                last_result = result.text().to_string();
            }
            started_time = Instant::now();
        }

        // Process completed speech segments from VAD
        while !vad.is_empty() {
            let segment = vad.front();

            // Use only the audio from speech start to current for final recognition
            // This ensures we don't include pre-speech audio that causes duplication
            let result = if !buffer.is_empty() && speech_start_index < buffer.len() {
                let speech_audio = &buffer[speech_start_index..];
                recognizer.transcribe(target_sample_rate as u32, speech_audio)
            } else if !buffer.is_empty() {
                recognizer.transcribe(target_sample_rate as u32, &buffer)
            } else {
                recognizer.transcribe(target_sample_rate as u32, &segment.samples)
            }?;

            // Display final result in default color with emotion if available
            println!("\n✅ Final: {}", result.text());

            vad.pop();
            // Clear buffer after processing segment to avoid overlap with next segment
            buffer.clear();
            offset = 0;
            speech_started = false;
            speech_start_index = 0;
            last_result.clear();
        }
    }

    println!("\nStopped.");
    Ok(())
}
