//! Commonly used middleware.

mod add_data;
#[cfg(feature = "compression")]
mod compression;
mod cors;
#[cfg(feature = "tracing")]
mod tracing;

pub use add_data::AddData;
#[cfg(feature = "compression")]
#[cfg_attr(docsrs, doc(cfg(feature = "compression")))]
pub use compression::{Compress, CompressionAlgo, Decompress};
pub use cors::Cors;

#[cfg(feature = "tracing")]
#[cfg_attr(docsrs, doc(cfg(feature = "tracing")))]
pub use self::tracing::Tracing;
use crate::endpoint::Endpoint;

/// Represents a middleware trait.
pub trait Middleware<E: Endpoint> {
    /// New endpoint type.
    ///
    /// If you don't know what type to use, then you can use [`Box<dyn
    /// Endpoint>`], which will bring some performance loss, but it is
    /// insignificant.
    type Output: Endpoint;

    /// Transform the input [`Endpoint`] to another one.
    fn transform(self, ep: E) -> Self::Output;
}
