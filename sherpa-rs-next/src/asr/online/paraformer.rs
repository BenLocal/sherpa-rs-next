use std::ffi::CString;

use sherpa_rs_next_macro::FromBaseConfig;

use crate::{as_c_string, asr::online::AsrOnlineBaseConfig};

#[derive(Debug, Default, FromBaseConfig)]
pub struct ParaformerAsrOnlineConfig {
    #[base_config(path = "src/asr/online/mod.rs")]
    base: AsrOnlineBaseConfig,

    encoder: Option<CString>,
    decoder: Option<CString>,
}

impl ParaformerAsrOnlineConfig {
    pub fn with_encoder(&mut self, encoder: &str) -> &mut Self {
        let encoder = as_c_string!(encoder);
        self.base.config.model_config.paraformer.encoder = encoder.as_ptr();
        self.encoder = Some(encoder);
        self
    }

    pub fn with_decoder(&mut self, decoder: &str) -> &mut Self {
        let decoder = as_c_string!(decoder);
        self.base.config.model_config.paraformer.decoder = decoder.as_ptr();
        self.decoder = Some(decoder);
        self
    }
}

impl AsRef<sherpa_rs_sys::SherpaOnnxOnlineRecognizerConfig> for ParaformerAsrOnlineConfig {
    fn as_ref(&self) -> &sherpa_rs_sys::SherpaOnnxOnlineRecognizerConfig {
        &self.base.config
    }
}

#[cfg(test)]
mod tests {
    use crate::const_ptr_to_string;

    use super::*;

    #[test]
    fn test_paraformer_asr_online_config() {
        let mut config = ParaformerAsrOnlineConfig::default();
        config
            .with_model_debug(true)
            .with_model_num_threads(1000)
            .with_model_provider("cpu")
            .with_model_type("paraformer")
            .with_encoder("/assets/encoder.onnx")
            .with_decoder("/assets/decoder.onnx");
        let config: &sherpa_rs_sys::SherpaOnnxOnlineRecognizerConfig = config.as_ref();
        assert_eq!(
            const_ptr_to_string!(config.model_config.paraformer.encoder),
            "/assets/encoder.onnx"
        );
        assert_eq!(
            const_ptr_to_string!(config.model_config.paraformer.decoder),
            "/assets/decoder.onnx"
        );

        assert_eq!(config.model_config.debug, 1);
        assert_eq!(config.model_config.num_threads, 1000);
        assert_eq!(const_ptr_to_string!(config.model_config.provider), "cpu");
        assert_eq!(
            const_ptr_to_string!(config.model_config.model_type),
            "paraformer"
        );
    }
}
