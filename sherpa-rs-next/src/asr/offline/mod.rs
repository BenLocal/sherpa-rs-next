use std::ffi::CString;

use crate::as_c_string;

pub mod paraformer;

#[derive(Debug, Default)]
pub struct AsrOfflineBaseConfig {
    config: sherpa_rs_sys::SherpaOnnxOfflineRecognizerConfig,
}

impl AsrOfflineBaseConfig {}

pub struct AsrOfflineRecognizer(*const sherpa_rs_sys::SherpaOnnxOfflineRecognizer);

impl AsrOfflineRecognizer {
    pub fn create<T>(config: T) -> anyhow::Result<Self>
    where
        T: AsRef<sherpa_rs_sys::SherpaOnnxOfflineRecognizerConfig>,
    {
        let config = config.as_ref();
        let recognizer = unsafe { sherpa_rs_sys::SherpaOnnxCreateOfflineRecognizer(config) };
        Ok(Self(recognizer))
    }

    pub fn transcribe(&mut self, sample_rate: u32, samples: &[f32]) -> () {
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
            let raw_result = result_ptr.read();
            //let result = ParaformerRecognizerResult::new(&raw_result);

            sherpa_rs_sys::SherpaOnnxDestroyOfflineRecognizerResult(result_ptr);
            sherpa_rs_sys::SherpaOnnxDestroyOfflineStream(stream);

            ()
        }
    }
}
