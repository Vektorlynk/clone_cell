//! This crate provides a [`Cell`] implementation that works with types whose `Clone`
//! implementations are guaranteed not to mutate the `Cell` content using the `&self`
//! reference. This is enforced with the provided [`PureClone`] trait, which is a
//! subtrait of [`Clone`] (and a logical supertrait of [`Copy`]). It is only
//! implemented for types with compliant `clone` methods.
//!
//! See the [`cell` module] documentation for more information on how to use it.
//!
//! # Background
//!
//! This crate was largely inspired by the Swift programming language's class
//! properties (fields in Rust speak), which have value semantics. In Swift, class
//! types themselves have reference semantics and are shared. But methods on class
//! types are mutating. My observation is that the Swift compiler is able to
//! guarantee memory safety in a single-threaded context because copy constructors
//! are not defined by the user. Intead, the compiler automatically generates ones
//! that simply perform a field-wise clone.
//!
//! In Rust, to enable interiorly mutating methods on a `struct` stored in an [`Rc`]
//! without the overhead of a [`RefCell`], we can wrap each of the fields in a
//! [`std::cell::Cell`]. But its [`get`] method is only implemented for types that
//! are `Copy`. This is because if the `clone` method obtains a reference to the
//! `Cell`'s interior, it may be able to mutate its state. This can cause undefined
//! behavior, as demonstrated in this [example].
//!
//! By restricting ourselves to a checked subset of `Clone` implementations that do
//! not exploit interior mutability to mutate the `Cell` content, it becomes possible
//! to provide a `Cell` with a `get` method that does not require `Copy` types.
//!
//! See the documentation for [`PureClone`] for a list of implemented types and more
//! details.
//!
//! [`Cell`]: cell::Cell
//! [`PureClone`]: clone::PureClone
//! [`cell` module]: cell
//! [`Rc`]: std::rc::Rc
//! [`RefCell`]: std::cell::RefCell
//! [`get`]: std::cell::Cell::get
//! [example]:
//! https://users.rust-lang.org/t/why-does-cell-require-copy-instead-of-clone/5769/3

// TODO: #![no_std]

pub mod cell;
pub mod clone;
//#[cfg(feature = "derive")]
//use clone_cell_derive as derive;
