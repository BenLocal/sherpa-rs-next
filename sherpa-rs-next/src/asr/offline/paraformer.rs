use std::ffi::CString;

use crate::{as_c_string, asr::offline::AsrOfflineBaseConfig};

#[derive(Debug, Default)]
pub struct ParaformerAsrOfflineConfig {
    base: AsrOfflineBaseConfig,
    model: Option<CString>,
}

crate::delegate_all_base_config_methods!(ParaformerAsrOfflineConfig);

impl ParaformerAsrOfflineConfig {
    pub fn with_model(&mut self, model: &str) -> &mut Self {
        let model = as_c_string!(model);
        self.base.config.model_config.paraformer.model = model.as_ptr();
        self.model = Some(model);
        self
    }
}

impl AsRef<sherpa_rs_sys::SherpaOnnxOfflineRecognizerConfig> for ParaformerAsrOfflineConfig {
    fn as_ref(&self) -> &sherpa_rs_sys::SherpaOnnxOfflineRecognizerConfig {
        &self.base.config
    }
}

#[cfg(test)]
mod tests {
    use crate::const_ptr_to_string;

    use super::*;

    #[test]
    fn test_paraformer_asr_offline_config() {
        let mut config = ParaformerAsrOfflineConfig::default();
        config
            .with_model("/assets/model.onnx")
            .with_model_debug(true)
            .with_model_num_threads(1000)
            .with_model_provider("cpu")
            .with_model_type("paraformer")
            .with_model_modeling_unit("char")
            .with_model_bpe_vocab("vocab.txt")
            .with_model_telespeech_ctc("ctc.onnx")
            .with_model_tokens("tokens.txt");
        let config: &sherpa_rs_sys::SherpaOnnxOfflineRecognizerConfig = config.as_ref();
        assert_eq!(
            const_ptr_to_string!(config.model_config.paraformer.model),
            "/assets/model.onnx"
        );
        assert_eq!(config.model_config.debug, 1);
        assert_eq!(config.model_config.num_threads, 1000);
        assert_eq!(const_ptr_to_string!(config.model_config.provider), "cpu");
        assert_eq!(
            const_ptr_to_string!(config.model_config.model_type),
            "paraformer"
        );
        assert_eq!(
            const_ptr_to_string!(config.model_config.modeling_unit),
            "char"
        );
        assert_eq!(
            const_ptr_to_string!(config.model_config.bpe_vocab),
            "vocab.txt"
        );
        assert_eq!(
            const_ptr_to_string!(config.model_config.telespeech_ctc),
            "ctc.onnx"
        );
        assert_eq!(
            const_ptr_to_string!(config.model_config.tokens),
            "tokens.txt"
        );
    }
}
