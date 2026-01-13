use std::ffi::CString;

use sherpa_rs_next_macro::FromBaseConfig;

use crate::{as_c_string, asr::online::AsrOnlineBaseConfig};

#[derive(Debug, Default, FromBaseConfig)]
pub struct ZipformerCtcAsrOnlineConfig {
    #[base_config(path = "src/asr/online/mod.rs")]
    base: AsrOnlineBaseConfig,

    model: Option<CString>,
}

impl ZipformerCtcAsrOnlineConfig {
    pub fn with_model(&mut self, model: &str) -> &mut Self {
        let model = as_c_string!(model);
        self.base.config.model_config.zipformer2_ctc.model = model.as_ptr();
        self.model = Some(model);
        self
    }
}

/// Zipformer CTC ASR Online Config
impl AsRef<sherpa_rs_sys::SherpaOnnxOnlineRecognizerConfig> for ZipformerCtcAsrOnlineConfig {
    fn as_ref(&self) -> &sherpa_rs_sys::SherpaOnnxOnlineRecognizerConfig {
        &self.base.config
    }
}
