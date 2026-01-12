use std::ffi::CString;

use crate::as_c_string;

pub mod kokoro;

pub type TTSConfig = Box<dyn AsRef<sherpa_rs_sys::SherpaOnnxOfflineTtsConfig>>;

#[derive(Debug, Default)]
pub struct TTSBaseConfig {
    config: sherpa_rs_sys::SherpaOnnxOfflineTtsConfig,

    rule_fsts: Option<CString>,
    rule_fars: Option<CString>,
}

impl TTSBaseConfig {
    pub fn with_rule_fsts(&mut self, rule_fsts: &str) -> &mut Self {
        let rule_fsts = as_c_string!(rule_fsts);
        self.config.rule_fsts = rule_fsts.as_ptr();
        self.rule_fsts = Some(rule_fsts);
        self
    }

    pub fn with_max_num_sentences(&mut self, max_num_sentences: i32) -> &mut Self {
        self.config.max_num_sentences = max_num_sentences;
        self
    }

    pub fn with_silence_scale(&mut self, silence_scale: f32) -> &mut Self {
        self.config.silence_scale = silence_scale;
        self
    }

    pub fn with_rule_fars(&mut self, rule_fars: &str) -> &mut Self {
        let rule_fars = as_c_string!(rule_fars);
        self.config.rule_fars = rule_fars.as_ptr();
        self.rule_fars = Some(rule_fars);
        self
    }
}

pub struct TTSAudio {
    samples: Vec<f32>,
    sample_rate: i32,
    duration: i32,
}

impl TTSAudio {
    pub fn samples(&self) -> &[f32] {
        &self.samples
    }

    pub fn sample_rate(&self) -> i32 {
        self.sample_rate
    }

    pub fn duration(&self) -> i32 {
        self.duration
    }
}

pub struct OfflineTTS(*const sherpa_rs_sys::SherpaOnnxOfflineTts);

impl OfflineTTS {
    pub fn create_with_config(config: TTSConfig) -> anyhow::Result<Self> {
        let config = config.as_ref();
        Self::create(config)
    }

    pub fn create<T>(config: T) -> anyhow::Result<Self>
    where
        T: AsRef<sherpa_rs_sys::SherpaOnnxOfflineTtsConfig>,
    {
        let config = config.as_ref();
        let tts = unsafe { sherpa_rs_sys::SherpaOnnxCreateOfflineTts(config) };
        Ok(Self(tts))
    }

    pub fn generate(&self, text: &str, sid: i32, speed: f32) -> anyhow::Result<TTSAudio> {
        let text = as_c_string!(text);
        unsafe {
            let audio_ptr =
                sherpa_rs_sys::SherpaOnnxOfflineTtsGenerate(self.0, text.as_ptr(), sid, speed);

            if audio_ptr.is_null() {
                sherpa_rs_sys::SherpaOnnxDestroyOfflineTtsGeneratedAudio(audio_ptr);
                return Err(anyhow::anyhow!("Failed to generate audio"));
            }

            let audio = audio_ptr.read();

            if audio.n.is_negative() {
                sherpa_rs_sys::SherpaOnnxDestroyOfflineTtsGeneratedAudio(audio_ptr);
                return Err(anyhow::anyhow!("no samples found"));
            }

            if audio.samples.is_null() {
                sherpa_rs_sys::SherpaOnnxDestroyOfflineTtsGeneratedAudio(audio_ptr);
                return Err(anyhow::anyhow!("no samples found"));
            }

            let samples = std::slice::from_raw_parts(audio.samples, audio.n as usize);
            let sample_rate = audio.sample_rate;
            let duration = (samples.len() as i32) / sample_rate;
            let res = TTSAudio {
                samples: samples.to_vec(),
                sample_rate,
                duration,
            };

            sherpa_rs_sys::SherpaOnnxDestroyOfflineTtsGeneratedAudio(audio_ptr);
            Ok(res)
        }
    }
}

impl Drop for OfflineTTS {
    fn drop(&mut self) {
        unsafe {
            sherpa_rs_sys::SherpaOnnxDestroyOfflineTts(self.0);
        }
    }
}
