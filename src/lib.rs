//! # DW23 — Wagon Load
//!
//! ```text
//! Load: 1650 lbs (ok)
//! Load: 2000 lbs (ok)
//! Load: 2002 lbs (overloaded)
//! ```
//!
//! Do not edit `src/main.rs`; the bugs are in the library files.

pub mod wagon;

use wagon::{Cargo, is_overloaded, total_weight};

/// Reports the weight of a load and whether the wagon can carry it.
///
/// ```
/// use dw23::wagon::Cargo;
///
/// let load = [Cargo::Flour(400), Cargo::Bullets(50), Cargo::Oxen(4), Cargo::Flour(1150)];
/// assert_eq!(dw23::describe_load(&load), "Load: 1650 lbs (ok)");
/// ```
///
/// A load exactly at capacity is fine:
///
/// ```
/// use dw23::wagon::Cargo;
///
/// let load = [Cargo::Flour(1900), Cargo::Bullets(50)];
/// assert_eq!(dw23::describe_load(&load), "Load: 2000 lbs (ok)");
/// ```
///
/// ```
/// use dw23::wagon::Cargo;
///
/// let load = [Cargo::Flour(2000), Cargo::Bullets(1)];
/// assert_eq!(dw23::describe_load(&load), "Load: 2002 lbs (overloaded)");
/// ```
pub fn describe_load(load: &[Cargo]) -> String {
    let total = total_weight(load);
    let status = if !is_overloaded(total) {
        "overloaded"
    } else {
        "ok"
    };
    format!("Load: {} lbs ({})", total, status)
}
