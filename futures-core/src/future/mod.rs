//! Futures.

use crate::task::{LocalWaker, Poll};
use core::pin::Pin;

pub use core::future::Future;

mod future_obj;
pub use self::future_obj::{FutureObj, LocalFutureObj, UnsafeFutureObj};

/// A convenience for futures that return `Result` values that includes
/// a variety of adapters tailored to such futures.
pub trait TryFuture {
    /// The type of successful values yielded by this future
    type Ok;

    /// The type of failures yielded by this future
    type Error;

    /// Poll this `TryFuture` as if it were a `Future`.
    ///
    /// This method is a stopgap for a compiler limitation that prevents us from
    /// directly inheriting from the `Future` trait; in the future it won't be
    /// needed.
    fn try_poll(
        self: Pin<&mut Self>,
        lw: &LocalWaker,
    ) -> Poll<Result<Self::Ok, Self::Error>>;
}

impl<F, T, E> TryFuture for F
    where F: Future<Output = Result<T, E>>
{
    type Ok = T;
    type Error = E;

    #[inline]
    fn try_poll(self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<F::Output> {
        self.poll(lw)
    }
}
