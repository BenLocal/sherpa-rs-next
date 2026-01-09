use std::ffi::CString;

use crate::{as_c_string, asr::offline::AsrOfflineBaseConfig};

pub struct ParaformerAsrOfflineConfig {
    base: AsrOfflineBaseConfig,
    model: Option<CString>,
}

impl ParaformerAsrOfflineConfig {}

impl Default for ParaformerAsrOfflineConfig {
    fn default() -> Self {
        Self {
            base: AsrOfflineBaseConfig::default(),
            model: None,
        }
    }
}

impl AsRef<sherpa_rs_sys::SherpaOnnxOfflineRecognizerConfig> for ParaformerAsrOfflineConfig {
    fn as_ref(&self) -> &sherpa_rs_sys::SherpaOnnxOfflineRecognizerConfig {
        &self.base.config
    }
}
