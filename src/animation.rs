//! Frame and animation abstractions
//!
//! Provides types that represent animations that can be shown
//! moving across the screen. Animations are composed of frames
//! and can be shown at different rates.

use crate::error::{Error, Result};

/// A struct representing a single frame of animation
pub struct Frame {
    pub text: Vec<String>,
}

impl Frame {
    /// Create a frame of animation from a single string
    fn from_str(text: &str) -> Result<Self> {
        if text.len() == 0 {
            return Err(Error::EmptyFrame);
        }

        Ok(Self {
            text: text
                .split("\n")
                .map(|line| line.to_string())
                .collect::<Vec<_>>(),
        })
    }

    /// Width of this frame
    ///
    /// The width of a frame is defined as the length of the longest
    /// line in the frame.
    fn width(&self) -> usize {
        self.text.iter().map(|line| line.len()).max().unwrap_or(0)
    }

    /// Height of this frame
    /// The height of the frame is defined as the number of lines in
    /// the frame
    fn height(&self) -> usize {
        self.text.len()
    }
}

/// A representation of a collection of frames which can be animated together
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
    ///
    /// # Arguments
    ///
    /// * `speed` - The speed of the animation, lower is faster
    /// * `text` - A collection of frames separated by two newlines
    pub fn new(speed: usize, text: &str) -> Result<Self> {
        if speed == 0 {
            return Err(Error::InvalidAnimationSpeed(speed));
        }

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_frame_creation() {
        let frame = Frame::from_str("simple contents");
        assert!(frame.is_ok());
    }

    #[test]
    fn frame_width_calc() {
        let frame = Frame::from_str("simple contents").unwrap();
        assert_eq!(frame.width(), 15);

        let frame = Frame::from_str(
            "some long line
more
short
lines",
        )
        .unwrap();

        assert_eq!(frame.width(), 14);
    }

    #[test]
    fn frame_height_calc() {
        let frame = Frame::from_str("simple contents").unwrap();
        assert_eq!(frame.height(), 1);

        let frame = Frame::from_str(
            "some long line
more
short
lines",
        )
        .unwrap();

        assert_eq!(frame.height(), 4);
    }

    const TEST_ANIMATION_TEXT: &'static str = "frame 1 body


frame 2 body
more text in frame two";

    #[test]
    fn basic_animation_creation() {
        let animation = Animation::new(1, TEST_ANIMATION_TEXT);

        assert!(animation.is_ok());
        assert_eq!(animation.unwrap().frames.len(), 2);
    }

    #[test]
    fn animation_creation_requires_frames() {
        let animation = Animation::new(1, "");
        assert!(animation.is_err());
    }

    #[test]
    fn animation_creation_requires_speed() {
        let animation = Animation::new(0, TEST_ANIMATION_TEXT);
        assert!(animation.is_err());
    }

    #[test]
    fn animation_height_calc() {
        let animation = Animation::new(1, TEST_ANIMATION_TEXT).unwrap();
        assert_eq!(animation.height(), 2)
    }

    #[test]
    fn animation_width_calc() {
        let animation = Animation::new(1, TEST_ANIMATION_TEXT).unwrap();
        assert_eq!(animation.width(), 22);
    }
}
