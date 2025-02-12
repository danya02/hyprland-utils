use std::sync::mpsc::{Receiver, SyncSender};

use rodio::{Sample, Source};

/// This struct wraps a stream.
/// When the stream is over, the struct sends a message to the provided channel.
pub struct StreamWrapper<T> {
    stream: T,
    channel: SyncSender<()>,
}

impl<T> StreamWrapper<T> {
    pub fn new(stream: T) -> (StreamWrapper<T>, Receiver<()>) {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        (
            StreamWrapper {
                stream,
                channel: sender,
            },
            receiver,
        )
    }
}

impl<Source, Samp> Iterator for StreamWrapper<Source>
where
    Source: Iterator<Item = Samp>,
    Samp: Sample,
{
    type Item = Samp;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stream.next() {
            Some(sample) => Some(sample),
            None => {
                // Deliberately ignoring any errors, because the first time there is no data available anymore,
                // we're going to be stopping the audio stream.
                let _ = self.channel.send(());
                None
            }
        }
    }
}

impl<Src, Samp> Source for StreamWrapper<Src>
where
    Src: Iterator<Item = Samp> + Source,
    Samp: Sample,
{
    fn channels(&self) -> u16 {
        self.stream.channels()
    }
    fn sample_rate(&self) -> u32 {
        self.stream.sample_rate()
    }
    fn total_duration(&self) -> Option<std::time::Duration> {
        self.stream.total_duration()
    }

    fn current_frame_len(&self) -> Option<usize> {
        self.stream.current_frame_len()
    }
}
