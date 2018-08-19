// Copyright 2013-2014 The Rust Project Developers.
// Copyright 2018 The Uuid Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Adapters for various formats for [`Uuid`]s
//!
//! [`Uuid`]: ../struct.Uuid.html

use prelude::*;

mod core_support;

/// An adaptor for formatting an [`Uuid`] as a hyphenated string.
///
/// Takes an owned instance of the [`Uuid`].
///
/// [`Uuid`]: ../struct.Uuid.html
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Hyphenated(Uuid);

/// An adaptor for formatting an [`Uuid`] as a hyphenated string.
///
/// Takes a reference of the [`Uuid`].
///
/// [`Uuid`]: ../struct.Uuid.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HyphenatedRef<'a>(&'a Uuid);

/// An adaptor for formatting an [`Uuid`] as a simple string.
///
/// Takes an owned instance of the [`Uuid`].
///
/// [`Uuid`]: ../struct.Uuid.html
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Simple(Uuid);

/// An adaptor for formatting an [`Uuid`] as a simple string.
///
/// Takes a reference of the [`Uuid`].
///
/// [`Uuid`]: ../struct.Uuid.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SimpleRef<'a>(&'a Uuid);

/// An adaptor for formatting an [`Uuid`] as a URN string.
///
/// Takes an owned instance of the [`Uuid`].
///
/// [`Uuid`]: ../struct.Uuid.html
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Urn(Uuid);

/// An adaptor for formatting an [`Uuid`] as a URN string.
///
/// Takes a reference of the [`Uuid`].
///
/// [`Uuid`]: ../struct.Uuid.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UrnRef<'a>(&'a Uuid);

impl Uuid {
    /// Creates a [`Hyphenated`] instance from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`Hyphenated`]: struct.Hyphenated.html
    // TODO: discuss to_ vs as_ vs into_
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_hyphenated(self) -> Hyphenated {
        Hyphenated::from_uuid(self)
    }

    /// Creates a [`Hyphenated`] instance from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`Hyphenated`]: struct.Hyphenated.html
    // TODO: discuss to_ vs as_ vs into_
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_hyphenated(self) -> Hyphenated {
        Hyphenated::from_uuid(self)
    }

    /// Creates a [`Hyphenated`Ref`] instance from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`Hyphenated`Ref`]: struct.HyphenatedRef.html
    // TODO(: discuss to_ vs as_ vs into_
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_hyphenated_ref(&self) -> HyphenatedRef {
        HyphenatedRef::from_uuid_ref(self)
    }

    /// Creates a [`Hyphenated`Ref`] instance from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`Hyphenated`Ref`]: struct.HyphenatedRef.html
    // TODO: discuss to_ vs as_ vs into_
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_hyphenated_ref(&self) -> HyphenatedRef {
        HyphenatedRef::from_uuid_ref(self)
    }

    /// Creates a [`Simple`] instance from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`Simple`]: struct.Simple.html
    // TODO: discuss to_ vs as_ vs into_
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_simple(self) -> Simple {
        Simple::from_uuid(self)
    }

    /// Creates a [`Simple`] instance from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`Simple`]: struct.Simple.html
    // TODO: discuss to_ vs as_ vs into_
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_simple(self) -> Simple {
        Simple::from_uuid(self)
    }

    /// Creates a [`UuidSimpleRef`] instance from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidSimpleRef`]: struct.SimpleRef.html
    // TODO: discuss to_ vs as_ vs into_
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_simple_ref(&self) -> SimpleRef {
        SimpleRef::from_uuid_ref(self)
    }

    /// Creates a [`UuidSimpleRef`] instance from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidSimpleRef`]: struct.SimpleRef.html
    // TODO: discuss to_ vs as_ vs into_
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_simple_ref(&self) -> SimpleRef {
        SimpleRef::from_uuid_ref(self)
    }

    /// Creates a [`Urn`] instance from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`Urn`]: struct.Urn.html
    // TODO: discuss to_ vs as_ vs into_
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_urn(self) -> Urn {
        Urn::from_uuid(self)
    }

    /// Creates a [`Urn`] instance from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`Urn`]: struct.Urn.html
    // TODO: discuss to_ vs as_ vs into_
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_urn(self) -> Urn {
        Urn::from_uuid(self)
    }

    /// Creates a [`UuidUrnRef`] instance from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidUrnRef`]: struct.UrnRef.html
    // TODO: discuss to_ vs as_ vs into_
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_urn_ref(&self) -> UrnRef {
        UrnRef::from_uuid_ref(self)
    }

    /// Creates a [`UuidUrnRef`] instance from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidUrnRef`]: struct.UrnRef.html
    // TODO: discuss to_ vs as_ vs into_
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_urn_ref(&self) -> UrnRef {
        UrnRef::from_uuid_ref(self)
    }
}

impl Hyphenated {
    /// The length of a hyphenated [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    pub const LENGTH: usize = 36;

    /// Creates a [`Hyphenated`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`Hyphenated`]: struct.Hyphenated.html
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid(uuid: Uuid) -> Self {
        Hyphenated(uuid)
    }

    /// Creates a [`Hyphenated`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`Hyphenated`]: struct.Hyphenated.html
    #[cfg(feature = "const_fn")]
    pub fn from_uuid(uuid: Uuid) -> Self {
        Hyphenated(uuid)
    }
}

impl<'a> HyphenatedRef<'a> {
    /// The length of a hyphenated [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    pub const LENGTH: usize = 36;

    /// Creates a [`Hyphenated`Ref`] from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`Hyphenated`Ref`]: struct.HyphenatedRef.html
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        HyphenatedRef(uuid)
    }

    /// Creates a [`Hyphenated`Ref`] from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`Hyphenated`Ref`]: struct.HyphenatedRef.html
    #[cfg(feature = "const_fn")]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        HyphenatedRef(uuid)
    }
}

impl Simple {
    /// The length of a simple [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    pub const LENGTH: usize = 32;

    /// Creates a [`Simple`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`Simple`]: struct.Simple.html
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid(uuid: Uuid) -> Self {
        Simple(uuid)
    }

    /// Creates a [`Simple`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`Simple`]: struct.Simple.html
    #[cfg(feature = "const_fn")]
    pub fn from_uuid(uuid: Uuid) -> Self {
        Simple(uuid)
    }
}

impl<'a> SimpleRef<'a> {
    /// The length of a simple [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    pub const LENGTH: usize = 32;

    /// Creates a [`UuidSimpleRef`] from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidSimpleRef`]: struct.SimpleRef.html
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        SimpleRef(uuid)
    }

    /// Creates a [`UuidSimpleRef`] from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidSimpleRef`]: struct.SimpleRef.html
    #[cfg(feature = "const_fn")]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        SimpleRef(uuid)
    }
}

impl Urn {
    /// The length of a URN [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    pub const LENGTH: usize = 45;

    /// Creates a [`Urn`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`Urn`]: struct.Urn.html
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid(uuid: Uuid) -> Self {
        Urn(uuid)
    }

    /// Creates a [`Urn`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`Urn`]: struct.Urn.html
    #[cfg(feature = "const_fn")]
    pub fn from_uuid(uuid: Uuid) -> Self {
        Urn(uuid)
    }
}

impl<'a> UrnRef<'a> {
    /// The length of a URN [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    pub const LENGTH: usize = 45;

    /// Creates a [`UuidUrnRef`] from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidUrnRef`]: struct.UrnRef.html
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        UrnRef(uuid)
    }

    /// Creates a [`UuidUrnRef`] from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidUrnRef`]: struct.UrnRef.html
    #[cfg(feature = "const_fn")]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        UrnRef(&uuid)
    }
}
