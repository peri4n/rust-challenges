pub mod y2015;
pub mod y2016;
pub mod y2017;
pub mod y2018;
pub mod y2019;
pub mod y2023;
pub mod y2024;
pub mod y2025;

/// A day's puzzle input, parsed once and shared by both parts.
pub trait Solution: Sized {
    type Fst;
    type Snd;

    fn parse(input: &str) -> Self;
    fn fst(&self) -> Self::Fst;
    fn snd(&self) -> Self::Snd;
}
