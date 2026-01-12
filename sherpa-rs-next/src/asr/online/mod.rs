use std::{cell::OnceCell, ffi::CString};

use crate::{as_c_string, asr::RecognizerJsonResult, const_ptr_to_string};

pub mod paraformer;

pub type AsrOnlineConfig = Box<dyn AsRef<sherpa_rs_sys::SherpaOnnxOnlineRecognizerConfig>>;

#[derive(Debug, Default)]
pub struct AsrOnlineBaseConfig {
    config: sherpa_rs_sys::SherpaOnnxOnlineRecognizerConfig,

    model_provider: Option<CString>,
    model_type: Option<CString>,
    model_tokens: Option<CString>,
}

impl AsrOnlineBaseConfig {
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

    pub fn with_model_tokens(&mut self, tokens: &str) -> &mut Self {
        let tokens = as_c_string!(tokens);
        self.config.model_config.tokens = tokens.as_ptr();
        self.model_tokens = Some(tokens);
        self
    }

    pub fn with_enable_endpoint(&mut self, enable_endpoint: bool) -> &mut Self {
        self.config.enable_endpoint = if enable_endpoint { 1 } else { 0 };
        self
    }

    pub fn with_rule1_min_trailing_silence(
        &mut self,
        rule1_min_trailing_silence: f32,
    ) -> &mut Self {
        self.config.rule1_min_trailing_silence = rule1_min_trailing_silence;
        self
    }

    pub fn with_rule2_min_trailing_silence(
        &mut self,
        rule2_min_trailing_silence: f32,
    ) -> &mut Self {
        self.config.rule2_min_trailing_silence = rule2_min_trailing_silence;
        self
    }

    pub fn with_rule3_min_utterance_length(
        &mut self,
        rule3_min_utterance_length: f32,
    ) -> &mut Self {
        self.config.rule3_min_utterance_length = rule3_min_utterance_length;
        self
    }
}

pub struct AsrOnlineResult {
    json_value_cache: OnceCell<anyhow::Result<RecognizerJsonResult>>,
    tokens: Vec<String>,
    text: String,
    timestamps: Vec<f32>,
    json: String,
    is_final: bool,
    segment_id: i32,
}

impl AsrOnlineResult {
    pub fn json(&self) -> &anyhow::Result<RecognizerJsonResult> {
        let cached = self.json_value_cache.get_or_init(|| {
            serde_json::from_str(&self.json)
                .map_err(|e| anyhow::anyhow!("Failed to parse json: {}", e))
        });

        cached
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn timestamps(&self) -> &Vec<f32> {
        &self.timestamps
    }

    pub fn tokens(&self) -> &Vec<String> {
        &self.tokens
    }

    pub fn is_final(&self) -> bool {
        self.is_final
    }

    pub fn segment_id(&self) -> i32 {
        self.segment_id
    }
}

impl From<sherpa_rs_sys::SherpaOnnxOnlineRecognizerResult> for AsrOnlineResult {
    fn from(value: sherpa_rs_sys::SherpaOnnxOnlineRecognizerResult) -> Self {
        let text_owned = const_ptr_to_string!(value.text, "".to_string());
        let json_owned = const_ptr_to_string!(value.json, "".to_string());
        let timestamps_owned = if value.timestamps.is_null() || value.count == 0 {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(value.timestamps, value.count as usize).to_vec() }
        };
        let mut tokens_owned = Vec::with_capacity(value.count as usize);
        if !value.tokens.is_null() && value.count > 0 {
            let mut next_token: *const i8 = value.tokens;
            for _ in 0..value.count {
                let token = unsafe { std::ffi::CStr::from_ptr(next_token) };
                tokens_owned.push(token.to_string_lossy().into_owned());
                next_token = next_token
                    .wrapping_byte_offset(token.to_bytes_with_nul().len().try_into().unwrap());
            }
        }
        Self {
            json_value_cache: OnceCell::new(),
            tokens: Vec::new(),
            text: text_owned,
            timestamps: timestamps_owned,
            json: json_owned,
            is_final: false,
            segment_id: 0,
        }
    }
}

pub struct AsrOnlineRecognizer {
    recognizer: *const sherpa_rs_sys::SherpaOnnxOnlineRecognizer,
    stream: *const sherpa_rs_sys::SherpaOnnxOnlineStream,
    segment_id: i32,
}

impl Drop for AsrOnlineRecognizer {
    fn drop(&mut self) {
        unsafe {
            sherpa_rs_sys::SherpaOnnxDestroyOnlineStream(self.stream);
            sherpa_rs_sys::SherpaOnnxDestroyOnlineRecognizer(self.recognizer);
        }
    }
}

impl AsrOnlineRecognizer {
    pub fn create_with_config(config: AsrOnlineConfig) -> anyhow::Result<Self> {
        //let config_ptr: *const sherpa_rs_sys::SherpaOnnxOnlineRecognizerConfig = (*config).as_ref();
        let config = config.as_ref();
        Self::create(config)
    }

    pub fn create<T>(config: T) -> anyhow::Result<Self>
    where
        T: AsRef<sherpa_rs_sys::SherpaOnnxOnlineRecognizerConfig>,
    {
        let config = config.as_ref();
        let recognizer = unsafe { sherpa_rs_sys::SherpaOnnxCreateOnlineRecognizer(config) };
        if recognizer.is_null() {
            return Err(anyhow::anyhow!("Failed to create online recognizer"));
        }
        let stream = unsafe { sherpa_rs_sys::SherpaOnnxCreateOnlineStream(recognizer) };
        if stream.is_null() {
            unsafe {
                sherpa_rs_sys::SherpaOnnxDestroyOnlineRecognizer(recognizer);
            }
            return Err(anyhow::anyhow!("Failed to create online Paraformer stream"));
        }
        Ok(Self {
            recognizer,
            stream,
            segment_id: 0,
        })
    }

    pub fn transcribe(
        &mut self,
        sample_rate: u32,
        samples: &[f32],
    ) -> anyhow::Result<AsrOnlineResult> {
        unsafe {
            sherpa_rs_sys::SherpaOnnxOnlineStreamAcceptWaveform(
                self.stream,
                sample_rate as i32,
                samples.as_ptr(),
                samples.len() as i32,
            );

            while sherpa_rs_sys::SherpaOnnxIsOnlineStreamReady(self.recognizer, self.stream) == 1 {
                sherpa_rs_sys::SherpaOnnxDecodeOnlineStream(self.recognizer, self.stream);
            }

            let result_ptr =
                sherpa_rs_sys::SherpaOnnxGetOnlineStreamResult(self.recognizer, self.stream);
            let raw_result = result_ptr.read();
            let mut result = AsrOnlineResult::from(raw_result);
            sherpa_rs_sys::SherpaOnnxDestroyOnlineRecognizerResult(result_ptr);

            if sherpa_rs_sys::SherpaOnnxOnlineStreamIsEndpoint(self.recognizer, self.stream) == 1 {
                self.segment_id += 1;
                sherpa_rs_sys::SherpaOnnxOnlineStreamReset(self.recognizer, self.stream);
                result.is_final = true;
            }

            result.segment_id = self.segment_id;
            Ok(result)
        }
    }
}
