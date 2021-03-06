use std::collections::VecDeque;

pub struct Reverb {
    delay: usize,
    decay: f32,
    buf: VecDeque<f32>,
}

impl Reverb {
    /// `delay_ms`: Reverb delay in ms
    ///
    /// `sample_rate`: Sample rate of samples
    ///
    /// `decay`: Strength of the reverb
    pub fn new(delay_ms: usize, sample_rate: usize, decay: f32) -> Self {
        Self {
            delay: delay_ms * sample_rate / 1000,
            decay,
            buf: VecDeque::new(),
        }
    }

    /// Applies a reverb effect to sound samples
    pub fn apply(&mut self, samples: &mut [f32]) {
        self.buf.extend(samples.iter());

        if self.buf.len() > self.delay {
            let count = std::cmp::min(self.buf.len() - self.delay, samples.len());
            for s in samples.iter_mut().take(count) {
                if let Some(sample) = self.buf.pop_front() {
                    *s += sample * self.decay;
                } else {
                    return;
                }
            }
        }
    }

    /// Clears the reverb buffer
    pub fn clear(&mut self) {
        self.buf.clear();
    }
}
