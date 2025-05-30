use bytes::Bytes;
use futures_util::{future, Stream, StreamExt};
use std::error::Error;

use super::{RealtimeSpeechToText, StreamResponse, StreamResponseWord};

pub use hypr_clova::realtime::interface as clova;

impl<S, E> RealtimeSpeechToText<S, E> for hypr_clova::realtime::Client {
    async fn transcribe(
        &mut self,
        input_stream: S,
    ) -> Result<
        Box<dyn Stream<Item = Result<StreamResponse, crate::Error>> + Send + Unpin>,
        crate::Error,
    >
    where
        S: Stream<Item = Result<Bytes, E>> + Send + Unpin + 'static,
        E: Error + Send + Sync + 'static,
    {
        let output_stream = self.from_audio(input_stream).await?;

        let stream = output_stream.filter_map(|item| {
            let item = match item {
                Err(e) => Some(Err(crate::Error::ClovaError(e.to_string()))),
                Ok(response) => match response {
                    clova::StreamResponse::TranscribeFailure(a) => Some(Err(
                        crate::Error::ClovaError(serde_json::to_string(&a).unwrap()),
                    )),
                    clova::StreamResponse::TranscribeSuccess(r) => Some(Ok(StreamResponse {
                        words: vec![StreamResponseWord {
                            text: r.transcription.text,
                            start: r.transcription.start_timestamp,
                            end: r.transcription.end_timestamp,
                        }],
                    })),
                    clova::StreamResponse::Config(_) => None,
                },
            };

            future::ready(item)
        });

        Ok(Box::from(Box::pin(stream)))
    }
}
