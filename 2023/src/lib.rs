#![feature(try_trait_v2)]

pub use resu::eyre;
pub type Result<T> = std::result::Result<T, resu::Report>;

pub trait FailableCovert<T, E>: std::ops::Try {
    fn res(self) -> std::result::Result<T, E>;
}

impl<T> FailableCovert<T, eyre::Error> for Option<T> {
    fn res(self) -> std::result::Result<T, eyre::Error> {
        self.ok_or(eyre::eyre!("Option had `None` value."))
    }
}
