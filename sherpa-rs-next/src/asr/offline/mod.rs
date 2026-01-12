use std::cell::OnceCell;
use std::ffi::CString;

use crate::{as_c_string, asr::RecognizerJsonResult, const_ptr_to_string};

pub mod paraformer;
pub mod sense_voice;

pub type AsrOfflineConfig = Box<dyn AsRef<sherpa_rs_sys::SherpaOnnxOfflineRecognizerConfig>>;

#[derive(Debug, Default)]
pub struct AsrOfflineBaseConfig {
    config: sherpa_rs_sys::SherpaOnnxOfflineRecognizerConfig,
    decoding_method: Option<CString>,
    hotwords_file: Option<CString>,
    rule_fsts: Option<CString>,
    rule_fars: Option<CString>,

    hr_dict_dir: Option<CString>,
    hr_lexicon: Option<CString>,
    hr_rule_fsts: Option<CString>,

    lm_model: Option<CString>,

    model_provider: Option<CString>,
    model_type: Option<CString>,
    model_modeling_unit: Option<CString>,
    model_bpe_vocab: Option<CString>,
    model_telespeech_ctc: Option<CString>,
    model_tokens: Option<CString>,
}

impl AsrOfflineBaseConfig {
    pub fn with_decoding_method(&mut self, decoding_method: &str) -> &mut Self {
        let decoding_method = as_c_string!(decoding_method);
        self.config.decoding_method = decoding_method.as_ptr();
        self.decoding_method = Some(decoding_method);
        self
    }

    pub fn with_max_active_paths(&mut self, max_active_paths: i32) -> &mut Self {
        self.config.max_active_paths = max_active_paths;
        self
    }

    pub fn with_hotwords_file(&mut self, hotwords_file: &str) -> &mut Self {
        let hotwords_file = as_c_string!(hotwords_file);
        self.config.hotwords_file = hotwords_file.as_ptr();
        self.hotwords_file = Some(hotwords_file);
        self
    }

    pub fn with_hotwords_score(&mut self, hotwords_score: f32) -> &mut Self {
        self.config.hotwords_score = hotwords_score;
        self
    }

    pub fn with_rule_fsts(&mut self, rule_fsts: &str) -> &mut Self {
        let rule_fsts = as_c_string!(rule_fsts);
        self.config.rule_fsts = rule_fsts.as_ptr();
        self.rule_fsts = Some(rule_fsts);
        self
    }

    pub fn with_rule_fars(&mut self, rule_fars: &str) -> &mut Self {
        let rule_fars = as_c_string!(rule_fars);
        self.config.rule_fars = rule_fars.as_ptr();
        self.rule_fars = Some(rule_fars);
        self
    }

    pub fn with_blank_penalty(&mut self, blank_penalty: f32) -> &mut Self {
        self.config.blank_penalty = blank_penalty;
        self
    }

    pub fn with_hr_dict_dir(&mut self, hr_dict_dir: &str) -> &mut Self {
        let hr_dict_dir = as_c_string!(hr_dict_dir);
        self.config.hr.dict_dir = hr_dict_dir.as_ptr();
        self.hr_dict_dir = Some(hr_dict_dir);
        self
    }

    pub fn with_hr_lexicon(&mut self, hr_lexicon: &str) -> &mut Self {
        let hr_lexicon = as_c_string!(hr_lexicon);
        self.config.hr.lexicon = hr_lexicon.as_ptr();
        self.hr_lexicon = Some(hr_lexicon);
        self
    }

    pub fn with_hr_rule_fsts(&mut self, hr_rule_fsts: &str) -> &mut Self {
        let hr_rule_fsts = as_c_string!(hr_rule_fsts);
        self.config.hr.rule_fsts = hr_rule_fsts.as_ptr();
        self.hr_rule_fsts = Some(hr_rule_fsts);
        self
    }

    pub fn with_lm_model(&mut self, lm_model: &str) -> &mut Self {
        let lm_model = as_c_string!(lm_model);
        self.config.lm_config.model = lm_model.as_ptr();
        self.lm_model = Some(lm_model);
        self
    }

    pub fn with_lm_scale(&mut self, scale: f32) -> &mut Self {
        self.config.lm_config.scale = scale;
        self
    }

    pub fn with_feat_sample_rate(&mut self, sample_rate: i32) -> &mut Self {
        self.config.feat_config.sample_rate = sample_rate;
        self
    }

    pub fn with_feat_feature_dim(&mut self, feature_dim: i32) -> &mut Self {
        self.config.feat_config.feature_dim = feature_dim;
        self
    }

    pub fn with_model_debug(&mut self, debug: bool) -> &mut Self {
        self.config.model_config.debug = if debug { 1 } else { 0 };
        self
    }

    pub fn with_model_num_threads(&mut self, num_threads: i32) -> &mut Self {
        self.config.model_config.num_threads = num_threads;
        self
    }

    pub fn with_model_provider(&mut self, provider: &str) -> &mut Self {
        let provider = as_c_string!(provider);
        self.config.model_config.provider = provider.as_ptr();
        self.model_provider = Some(provider);
        self
    }

    pub fn with_model_type(&mut self, model_type: &str) -> &mut Self {
        let model_type = as_c_string!(model_type);
        self.config.model_config.model_type = model_type.as_ptr();
        self.model_type = Some(model_type);
        self
    }

    pub fn with_model_modeling_unit(&mut self, modeling_unit: &str) -> &mut Self {
        let modeling_unit = as_c_string!(modeling_unit);
        self.config.model_config.modeling_unit = modeling_unit.as_ptr();
        self.model_modeling_unit = Some(modeling_unit);
        self
    }

    pub fn with_model_bpe_vocab(&mut self, bpe_vocab: &str) -> &mut Self {
        let bpe_vocab = as_c_string!(bpe_vocab);
        self.config.model_config.bpe_vocab = bpe_vocab.as_ptr();
        self.model_bpe_vocab = Some(bpe_vocab);
        self
    }

    pub fn with_model_telespeech_ctc(&mut self, telespeech_ctc: &str) -> &mut Self {
        let telespeech_ctc = as_c_string!(telespeech_ctc);
        self.config.model_config.telespeech_ctc = telespeech_ctc.as_ptr();
        self.model_telespeech_ctc = Some(telespeech_ctc);
        self
    }

    pub fn with_model_tokens(&mut self, tokens: &str) -> &mut Self {
        let tokens = as_c_string!(tokens);
        self.config.model_config.tokens = tokens.as_ptr();
        self.model_tokens = Some(tokens);
        self
    }
}

pub struct AsrOfflineResult {
    json_value_cache: OnceCell<anyhow::Result<RecognizerJsonResult>>,
    tokens: Vec<String>,
    text: String,
    lang: String,
    timestamps: Vec<f32>,
    json: String,
}

impl AsrOfflineResult {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn timestamps(&self) -> &Vec<f32> {
        &self.timestamps
    }

    pub fn lang(&self) -> &str {
        &self.lang
    }

    pub fn tokens(&self) -> &Vec<String> {
        &self.tokens
    }

    pub fn json(&self) -> &anyhow::Result<RecognizerJsonResult> {
        let cached = self.json_value_cache.get_or_init(|| {
            serde_json::from_str(&self.json)
                .map_err(|e| anyhow::anyhow!("Failed to parse json: {}", e))
        });

        cached
    }
}

impl From<sherpa_rs_sys::SherpaOnnxOfflineRecognizerResult> for AsrOfflineResult {
    fn from(result: sherpa_rs_sys::SherpaOnnxOfflineRecognizerResult) -> Self {
        let text_owned = const_ptr_to_string!(result.text, "".to_string());
        let lang_owned = const_ptr_to_string!(result.lang, "".to_string());
        let json_owned = const_ptr_to_string!(result.json, "".to_string());
        let timestamps_owned = if result.timestamps.is_null() || result.count == 0 {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(result.timestamps, result.count as usize).to_vec() }
        };

        let mut tokens_owned = Vec::with_capacity(result.count as usize);
        if !result.tokens.is_null() && result.count > 0 {
            let mut next_token: *const i8 = result.tokens;
            for _ in 0..result.count {
                let token = unsafe { std::ffi::CStr::from_ptr(next_token) };
                tokens_owned.push(token.to_string_lossy().into_owned());
                next_token = next_token
                    .wrapping_byte_offset(token.to_bytes_with_nul().len().try_into().unwrap());
            }
        }

        Self {
            json_value_cache: OnceCell::new(),
            tokens: tokens_owned,
            text: text_owned,
            lang: lang_owned,
            timestamps: timestamps_owned,
            json: json_owned,
        }
    }
}

pub struct AsrOfflineRecognizer(*const sherpa_rs_sys::SherpaOnnxOfflineRecognizer);

impl AsrOfflineRecognizer {
    pub fn create_with_config(config: AsrOfflineConfig) -> anyhow::Result<Self> {
        let config = config.as_ref();
        Self::create(config)
    }

    pub fn create<T>(config: T) -> anyhow::Result<Self>
    where
        T: AsRef<sherpa_rs_sys::SherpaOnnxOfflineRecognizerConfig>,
    {
        let config = config.as_ref();
        let recognizer = unsafe { sherpa_rs_sys::SherpaOnnxCreateOfflineRecognizer(config) };
        Ok(Self(recognizer))
    }

    pub fn transcribe(
        &self,
        sample_rate: u32,
        samples: &[f32],
    ) -> anyhow::Result<AsrOfflineResult> {
        unsafe {
            let stream = sherpa_rs_sys::SherpaOnnxCreateOfflineStream(self.0);
            sherpa_rs_sys::SherpaOnnxAcceptWaveformOffline(
                stream,
                sample_rate as i32,
                samples.as_ptr(),
                samples.len() as i32,
            );
            sherpa_rs_sys::SherpaOnnxDecodeOfflineStream(self.0, stream);
            let result_ptr = sherpa_rs_sys::SherpaOnnxGetOfflineStreamResult(stream);
            if result_ptr.is_null() {
                sherpa_rs_sys::SherpaOnnxDestroyOfflineStream(stream);
                return Err(anyhow::anyhow!("Failed to get offline stream result"));
            }
            let raw_result = result_ptr.read();
            let result = AsrOfflineResult::from(raw_result);
            sherpa_rs_sys::SherpaOnnxDestroyOfflineRecognizerResult(result_ptr);
            sherpa_rs_sys::SherpaOnnxDestroyOfflineStream(stream);
            Ok(result)
        }
    }
}
