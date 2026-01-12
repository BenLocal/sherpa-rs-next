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

pub fn write_audio_file(path: &str, samples: &[f32], sample_rate: u32) -> anyhow::Result<()> {
    // Create a WAV file writer
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(path, spec)?;

    // Convert samples from f32 to i16 and write them to the WAV file
    for &sample in samples {
        let scaled_sample =
            (sample * (i16::MAX as f32)).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
        writer.write_sample(scaled_sample)?;
    }

    writer.finalize()?;
    Ok(())
}
