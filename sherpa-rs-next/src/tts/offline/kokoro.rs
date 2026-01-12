use std::ffi::CString;

use sherpa_rs_next_macro::FromBaseConfig;

use crate::{as_c_string, tts::offline::TTSBaseConfig};

#[derive(Debug, Default, FromBaseConfig)]
pub struct KokoroTTSConfig {
    #[base_config(path = "src/tts/offline/mod.rs")]
    base: TTSBaseConfig,

    model: Option<CString>,
    voices: Option<CString>,
    tokens: Option<CString>,
    data_dir: Option<CString>,
    dict_dir: Option<CString>,
    lexicon: Option<CString>,
    lang: Option<CString>,
}

impl KokoroTTSConfig {
    pub fn with_model(&mut self, model: &str) -> &mut Self {
        let model = as_c_string!(model);
        self.base.config.model.kokoro.model = model.as_ptr();
        self.model = Some(model);
        self
    }

    pub fn with_voices(&mut self, voices: &str) -> &mut Self {
        let voices = as_c_string!(voices);
        self.base.config.model.kokoro.voices = voices.as_ptr();
        self.voices = Some(voices);
        self
    }

    pub fn with_tokens(&mut self, tokens: &str) -> &mut Self {
        let tokens = as_c_string!(tokens);
        self.base.config.model.kokoro.tokens = tokens.as_ptr();
        self.tokens = Some(tokens);
        self
    }

    pub fn with_data_dir(&mut self, data_dir: &str) -> &mut Self {
        let data_dir = as_c_string!(data_dir);
        self.base.config.model.kokoro.data_dir = data_dir.as_ptr();
        self.data_dir = Some(data_dir);
        self
    }

    pub fn with_length_scale(&mut self, length_scale: f32) -> &mut Self {
        self.base.config.model.kokoro.length_scale = length_scale;
        self
    }

    pub fn with_dict_dir(&mut self, dict_dir: &str) -> &mut Self {
        let dict_dir = as_c_string!(dict_dir);
        self.base.config.model.kokoro.dict_dir = dict_dir.as_ptr();
        self.dict_dir = Some(dict_dir);
        self
    }

    pub fn with_lexicon(&mut self, lexicon: &str) -> &mut Self {
        let lexicon = as_c_string!(lexicon);
        self.base.config.model.kokoro.lexicon = lexicon.as_ptr();
        self.lexicon = Some(lexicon);
        self
    }

    pub fn with_lang(&mut self, lang: &str) -> &mut Self {
        let lang = as_c_string!(lang);
        self.base.config.model.kokoro.lang = lang.as_ptr();
        self.lang = Some(lang);
        self
    }
}
