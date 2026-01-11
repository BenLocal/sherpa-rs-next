use std::ffi::CString;

use crate::as_c_string;

pub mod silero;
pub mod ten;

#[derive(Debug, Default)]
pub struct VadBaseConfig {
    config: sherpa_rs_sys::SherpaOnnxVadModelConfig,
    provider: Option<CString>,
}

impl VadBaseConfig {
    pub fn with_debug(&mut self, debug: bool) -> &mut Self {
        self.config.debug = if debug { 1 } else { 0 };
        self
    }

    pub fn with_sample_rate(&mut self, sample_rate: i32) -> &mut Self {
        self.config.sample_rate = sample_rate;
        self
    }

    pub fn with_num_threads(&mut self, num_threads: i32) -> &mut Self {
        self.config.num_threads = num_threads;
        self
    }

    pub fn with_provider(&mut self, provider: &str) -> &mut Self {
        let provider = as_c_string!(provider);
        self.config.provider = provider.as_ptr();
        self.provider = Some(provider);
        self
    }
}

#[derive(Debug)]
pub struct SpeechSegment {
    pub start: i32,
    pub samples: Vec<f32>,
}

pub struct Vad(*const sherpa_rs_sys::SherpaOnnxVoiceActivityDetector);

impl Vad {
    pub fn create<T>(config: T, buffer_size_in_seconds: f32) -> anyhow::Result<Self>
    where
        T: AsRef<sherpa_rs_sys::SherpaOnnxVadModelConfig>,
    {
        let config = config.as_ref();
        let vad = unsafe {
            sherpa_rs_sys::SherpaOnnxCreateVoiceActivityDetector(config, buffer_size_in_seconds)
        };

        if vad.is_null() {
            return Err(anyhow::anyhow!("Failed to create voice activity detector"));
        }
        Ok(Self(vad))
    }

    pub fn is_empty(&mut self) -> bool {
        unsafe { sherpa_rs_sys::SherpaOnnxVoiceActivityDetectorEmpty(self.0) == 1 }
    }

    pub fn front(&mut self) -> SpeechSegment {
        unsafe {
            let segment_ptr = sherpa_rs_sys::SherpaOnnxVoiceActivityDetectorFront(self.0);
            let raw_segment = segment_ptr.read();
            let samples: &[f32] =
                std::slice::from_raw_parts(raw_segment.samples, raw_segment.n as usize);

            let segment = SpeechSegment {
                samples: samples.to_vec(),
                start: raw_segment.start,
            };

            // Free
            sherpa_rs_sys::SherpaOnnxDestroySpeechSegment(segment_ptr);

            segment
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            sherpa_rs_sys::SherpaOnnxVoiceActivityDetectorFlush(self.0);
        }
    }

    pub fn accept_waveform(&mut self, mut samples: Vec<f32>) {
        let samples_ptr = samples.as_mut_ptr();
        let samples_length = samples.len();
        unsafe {
            sherpa_rs_sys::SherpaOnnxVoiceActivityDetectorAcceptWaveform(
                self.0,
                samples_ptr,
                samples_length.try_into().unwrap(),
            );
        };
    }

    pub fn pop(&mut self) {
        unsafe {
            sherpa_rs_sys::SherpaOnnxVoiceActivityDetectorPop(self.0);
        }
    }

    pub fn is_speech(&mut self) -> bool {
        unsafe { sherpa_rs_sys::SherpaOnnxVoiceActivityDetectorDetected(self.0) == 1 }
    }

    pub fn clear(&mut self) {
        unsafe {
            sherpa_rs_sys::SherpaOnnxVoiceActivityDetectorClear(self.0);
        }
    }
}

impl Drop for Vad {
    fn drop(&mut self) {
        unsafe {
            if !self.0.is_null() {
                sherpa_rs_sys::SherpaOnnxDestroyVoiceActivityDetector(self.0);
                self.0 = std::ptr::null();
            }
        }
    }
}
