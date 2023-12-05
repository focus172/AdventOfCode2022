#![feature(try_trait_v2)]

pub use std::io;
use std::ops::FromResidual;

pub struct W<T>(T);
impl<T> std::ops::Try for W<Option<T>> {
    type Output = T;

    type Residual = eyre::Error;

    fn from_output(output: Self::Output) -> Self {
        W(Some(output))
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self.0 {
            Some(t) => std::ops::ControlFlow::Continue(t),
            None => std::ops::ControlFlow::Break(eyre::eyre!("Option had `None` value")),
        }
    }
}
impl<T> FromResidual for W<Option<T>> {
    fn from_residual(_: <Self as std::ops::Try>::Residual) -> Self {
        W(None)
    }
}
pub trait Wrap: Sized {
    fn w(self) -> W<Self>;
}

impl<T> Wrap for T {
    fn w(self) -> W<Self> {
        W(self)
    }
}

pub trait OptionResult<T, E>: std::ops::Try {
    fn res(self) -> Result<T, E>;
}

impl<T> OptionResult<T, eyre::Error> for Option<T> {
    fn res(self) -> Result<T, eyre::Error> {
        self.ok_or(eyre::eyre!("Option had `None` value."))
    }
}
