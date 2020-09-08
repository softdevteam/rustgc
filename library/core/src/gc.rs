#![unstable(feature = "gc", issue = "none")]
#![allow(missing_docs)]

#[unstable(feature = "gc", issue = "none")]
#[cfg_attr(not(bootstrap), lang = "manageable_contents")]
/// This trait can be implemented on types where it is safe to allow the allow the collector to
/// free its memory and omit the drop method. This prevents the need to register a finalizer when
/// managed by the Gc which is expensive.
pub trait ManageableContents {}
