use std::ffi::CString;

use sherpa_rs_next_macro::FromBaseConfig;

use crate::{as_c_string, asr::offline::AsrOfflineBaseConfig};

#[derive(Debug, Default, FromBaseConfig)]
pub struct SenseVoiceAsrOfflineConfig {
    #[base_config(path = "src/asr/offline/mod.rs")]
    base: AsrOfflineBaseConfig,
    model: Option<CString>,
    language: Option<CString>,
}

impl SenseVoiceAsrOfflineConfig {
    pub fn with_model(&mut self, model: &str) -> &mut Self {
        let model = as_c_string!(model);
        self.base.config.model_config.sense_voice.model = model.as_ptr();
        self.model = Some(model);
        self
    }

    pub fn with_language(&mut self, language: &str) -> &mut Self {
        let language = as_c_string!(language);
        self.base.config.model_config.sense_voice.language = language.as_ptr();
        self.language = Some(language);
        self
    }

    pub fn with_use_itn(&mut self, use_itn: bool) -> &mut Self {
        self.base.config.model_config.sense_voice.use_itn = if use_itn { 1 } else { 0 };
        self
    }
}

impl AsRef<sherpa_rs_sys::SherpaOnnxOfflineRecognizerConfig> for SenseVoiceAsrOfflineConfig {
    fn as_ref(&self) -> &sherpa_rs_sys::SherpaOnnxOfflineRecognizerConfig {
        &self.base.config
    }
}
