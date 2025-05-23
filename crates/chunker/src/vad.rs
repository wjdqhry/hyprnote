// https://github.com/floneum/floneum/blob/52967ae/interfaces/kalosm-sound/src/transform/voice_audio_detector.rs

use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use futures_util::Stream;
use kalosm_sound::{rodio::buffer::SamplesBuffer, AsyncSource};
use silero::{VadConfig, VadSession, VadTransition};

pub struct VoiceActivityRechunker<S> {
    source: S,
    vad: VadSession,
    buffer: Vec<f32>,
    chunk_size: usize,
    sample_rate: usize,
}

impl<S> VoiceActivityRechunker<S>
where
    S: AsyncSource + Unpin,
{
    pub fn new(source: S) -> Self {
        let sample_rate = source.sample_rate() as usize;
        let chunk_size = 30 * sample_rate / 1000; // 30ms

        let vad_config = VadConfig {
            sample_rate,
            positive_speech_threshold: 0.4,
            negative_speech_threshold: 0.25,
            post_speech_pad: Duration::from_millis(100),
            pre_speech_pad: Duration::from_millis(500),
            redemption_time: Duration::from_millis(500),
            min_speech_time: Duration::from_millis(90),
        };
        let vad = VadSession::new(vad_config).unwrap();

        Self {
            source,
            vad,
            buffer: Vec::with_capacity(chunk_size * 2),
            chunk_size,
            sample_rate,
        }
    }
}

impl<S: AsyncSource + Unpin> Stream for VoiceActivityRechunker<S> {
    type Item = SamplesBuffer<f32>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        while this.buffer.len() < this.chunk_size {
            let stream = this.source.as_stream();
            let mut stream = std::pin::pin!(stream);

            let sample = match stream.as_mut().poll_next(cx) {
                Poll::Ready(Some(sample)) => sample,
                Poll::Ready(None) if !this.buffer.is_empty() => {
                    let data = std::mem::take(&mut this.buffer);
                    return Poll::Ready(Some(SamplesBuffer::new(1, this.sample_rate as u32, data)));
                }
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => return Poll::Pending,
            };

            this.buffer.push(sample);
        }

        let data = this
            .buffer
            .drain(..this.chunk_size.min(this.buffer.len()))
            .collect::<Vec<_>>();

        let transitions = match this.vad.process(&data) {
            Ok(transitions) => transitions,
            Err(e) => {
                tracing::error!("vad_error: {}", e);
                return Poll::Ready(None);
            }
        };

        for transition in transitions {
            if let VadTransition::SpeechEnd { samples, .. } = transition {
                return Poll::Ready(Some(SamplesBuffer::new(
                    1,
                    this.sample_rate as u32,
                    samples,
                )));
            }
        }

        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::StreamExt;

    #[tokio::test]
    async fn test_voice_activity_rechunker() {
        let audio_source = rodio::Decoder::new_wav(std::io::BufReader::new(
            std::fs::File::open(hypr_data::english_1::AUDIO_PATH).unwrap(),
        ))
        .unwrap();

        let rechunker = VoiceActivityRechunker::new(audio_source);
        let chunks = rechunker.collect::<Vec<_>>().await;

        let mut i = 0;
        for chunk in chunks {
            let spec = hound::WavSpec {
                channels: 1,
                sample_rate: 16000,
                bits_per_sample: 32,
                sample_format: hound::SampleFormat::Float,
            };
            let file = std::fs::File::create(format!("chunk_{}.wav", i)).unwrap();
            let mut writer = hound::WavWriter::new(file, spec).unwrap();
            for sample in chunk {
                writer.write_sample(sample).unwrap();
            }
            writer.finalize().unwrap();

            i += 1;
        }
    }
}
