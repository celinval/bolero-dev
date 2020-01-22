pub use bolero_generator::{
    driver::{Driver, DriverMode},
    TypeGenerator, ValueGenerator,
};
pub use failure::Error;

pub mod panic;
pub mod rng;
pub mod shrink;
mod test;
pub use test::*;

pub mod test_failure;
pub use crate::test_failure::TestFailure;

mod test_input;
pub use test_input::*;

mod target_location;
#[doc(hidden)]
pub use target_location::TargetLocation;

mod test_result;
pub use test_result::*;

pub trait Engine<T: Test> {
    type Output;

    fn set_driver_mode(&mut self, mode: DriverMode);
    fn run(self, test: T) -> Self::Output;
}

// TODO change this to `!` when stabilized
#[doc(hidden)]
pub type Never = ();
