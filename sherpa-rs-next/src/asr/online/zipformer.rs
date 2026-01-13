use std::ffi::CString;

use sherpa_rs_next_macro::FromBaseConfig;

use crate::{as_c_string, asr::online::AsrOnlineBaseConfig};

#[derive(Debug, Default, FromBaseConfig)]
pub struct ZipformerAsrOnlineConfig {
    #[base_config(path = "src/asr/online/mod.rs")]
    base: AsrOnlineBaseConfig,

    encoder: Option<CString>,
    decoder: Option<CString>,
    joiner: Option<CString>,
}

impl ZipformerAsrOnlineConfig {
    pub fn with_encoder(&mut self, encoder: &str) -> &mut Self {
        let encoder = as_c_string!(encoder);
        self.base.config.model_config.transducer.encoder = encoder.as_ptr();
        self.encoder = Some(encoder);
        self
    }

    pub fn with_decoder(&mut self, decoder: &str) -> &mut Self {
        let decoder = as_c_string!(decoder);
        self.base.config.model_config.transducer.decoder = decoder.as_ptr();
        self.decoder = Some(decoder);
        self
    }

    pub fn with_joiner(&mut self, joiner: &str) -> &mut Self {
        let joiner = as_c_string!(joiner);
        self.base.config.model_config.transducer.joiner = joiner.as_ptr();
        self.joiner = Some(joiner);
        self
    }
}

impl AsRef<sherpa_rs_sys::SherpaOnnxOnlineRecognizerConfig> for ZipformerAsrOnlineConfig {
    fn as_ref(&self) -> &sherpa_rs_sys::SherpaOnnxOnlineRecognizerConfig {
        &self.base.config
    }
}
