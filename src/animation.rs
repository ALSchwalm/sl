use crate::error::{Error, Result};

/// A struct representing a single frame of animation
pub struct Frame {
    pub text: Vec<String>,
}

impl Frame {
    /// Create a frame of animation from a single string
    fn from_str(text: &str) -> Result<Self> {
        Ok(Self {
            text: text
                .split("\n")
                .map(|line| line.to_string())
                .collect::<Vec<_>>(),
        })
    }

    fn width(&self) -> usize {
        self.text.iter().map(|line| line.len()).max().unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.text.len()
    }
}

pub struct Animation {
    frames: Vec<Frame>,
    current_frame_idx: usize,
    speed: usize,
    current_step: usize,
}

impl Animation {
    /// Create a new animation from a string
    ///
    /// Frames are expected to be delimited by two newlines. An
    /// error is returned if no frames are found
    pub fn from_str(speed: usize, text: &str) -> Result<Self> {
        let frames = text
            .split("\n\n\n")
            .map(|block| Frame::from_str(block))
            .collect::<Result<Vec<_>>>()?;

        if frames.len() == 0 {
            return Err(Error::EmptyAnimation);
        }

        Ok(Self {
            frames,
            speed,
            current_frame_idx: 0,
            current_step: 0,
        })
    }

    /// Advance the animation. This may update the current frame depending
    /// on the speed of the animation
    pub fn step(&mut self) {
        self.current_step += 1;
        if self.current_step == self.speed {
            self.current_frame_idx = (self.current_frame_idx + 1) % self.frames.len();
            self.current_step = 0;
        }
    }

    /// Get the current frame of the animation
    pub fn current_frame(&self) -> &Frame {
        &self.frames[self.current_frame_idx]
    }

    /// The maximum width of any frame in this animation
    pub fn width(&self) -> usize {
        self.frames
            .iter()
            .map(|frame| frame.width())
            .max()
            .expect("Unable to get frame width")
    }

    /// The maximum height of any frame in this animation
    pub fn height(&self) -> usize {
        self.frames
            .iter()
            .map(|frame| frame.height())
            .max()
            .expect("Unable to get frame height")
    }
}
