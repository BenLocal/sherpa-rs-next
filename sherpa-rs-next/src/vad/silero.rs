use std::ffi::CString;

use crate::{as_c_string, vad::VadBaseConfig};

#[derive(Debug, Default)]
pub struct SileroVadConfig {
    base: VadBaseConfig,
    model: Option<CString>,
}

impl SileroVadConfig {
    pub fn with_model(&mut self, model: &str) -> &mut Self {
        let model = as_c_string!(model);
        self.base.config.ten_vad.model = model.as_ptr();
        self.model = Some(model);
        self
    }

    pub fn with_threshold(&mut self, threshold: f32) -> &mut Self {
        self.base.config.ten_vad.threshold = threshold;
        self
    }

    pub fn with_min_silence_duration(&mut self, min_silence_duration: f32) -> &mut Self {
        self.base.config.ten_vad.min_silence_duration = min_silence_duration;
        self
    }

    pub fn with_min_speech_duration(&mut self, min_speech_duration: f32) -> &mut Self {
        self.base.config.ten_vad.min_speech_duration = min_speech_duration;
        self
    }

    pub fn with_max_speech_duration(&mut self, max_speech_duration: f32) -> &mut Self {
        self.base.config.ten_vad.max_speech_duration = max_speech_duration;
        self
    }

    pub fn with_window_size(&mut self, window_size: i32) -> &mut Self {
        self.base.config.ten_vad.window_size = window_size;
        self
    }
}

impl SileroVadConfig {
    crate::delegate_method!(debug, bool);
    crate::delegate_method!(sample_rate, i32);
    crate::delegate_method!(num_threads, i32);
    crate::delegate_method!(provider, &str);
}

impl AsRef<sherpa_rs_sys::SherpaOnnxVadModelConfig> for SileroVadConfig {
    fn as_ref(&self) -> &sherpa_rs_sys::SherpaOnnxVadModelConfig {
        &self.base.config
    }
}

#[cfg(test)]
mod tests {
    use crate::const_ptr_to_string;

    use super::*;

    #[test]
    fn test_ten_vad_config() {
        let mut config = SileroVadConfig::default();
        config
            .with_debug(true)
            .with_num_threads(1000)
            .with_max_speech_duration(10.0)
            .with_model("/assets/model.onnx");
        let config: &sherpa_rs_sys::SherpaOnnxVadModelConfig = config.as_ref();
        assert_eq!(config.debug, 1);
        assert_eq!(config.num_threads, 1000);
        assert_eq!(config.ten_vad.max_speech_duration, 10.0);
        assert_eq!(
            const_ptr_to_string!(config.ten_vad.model),
            "/assets/model.onnx"
        );
    }
}
