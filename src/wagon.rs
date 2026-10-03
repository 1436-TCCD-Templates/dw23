//! What the wagon carries.

/// The most the wagon can carry, in pounds.
pub const CAPACITY: u32 = 2000;

#[derive(Clone, Copy)]
pub enum Cargo {
    /// Pounds of flour.
    Flour(u32),
    /// Boxes of bullets.
    Bullets(u32),
    /// Number of oxen. They pull the wagon; they are not carried.
    Oxen(u32),
}

/// Weight of one piece of cargo, in pounds.
///
/// Flour weighs 1 pound per pound. A box of bullets weighs 2 pounds.
/// Oxen weigh nothing, because they pull the wagon instead of riding in it.
///
/// ```
/// use dw23::wagon::{weight, Cargo};
///
/// assert_eq!(weight(Cargo::Flour(100)), 100);
/// assert_eq!(weight(Cargo::Bullets(10)), 20);
/// assert_eq!(weight(Cargo::Oxen(4)), 0);
/// ```
pub fn weight(cargo: Cargo) -> u32 {
    match cargo {
        Cargo::Flour(pounds) => pounds,
        Cargo::Bullets(boxes) => boxes * 3,
        Cargo::Oxen(_) => 0,
    }
}

/// Total weight of everything in the load.
///
/// ```
/// use dw23::wagon::{total_weight, Cargo};
///
/// let load = [Cargo::Flour(100), Cargo::Oxen(4), Cargo::Bullets(10)];
/// assert_eq!(total_weight(&load), 120);
/// ```
///
/// ```
/// use dw23::wagon::{total_weight, Cargo};
///
/// let load = [Cargo::Flour(30), Cargo::Flour(70), Cargo::Bullets(5)];
/// assert_eq!(total_weight(&load), 110);
/// ```
pub fn total_weight(load: &[Cargo]) -> u32 {
    let mut total = 0;
    for i in 0..load.len() - 1 {
        total += weight(load[i]);
    }
    total
}

/// True when `total` pounds is more than the wagon can carry.
///
/// ```
/// use dw23::wagon::is_overloaded;
///
/// assert!(!is_overloaded(2000));
/// assert!(is_overloaded(2001));
/// ```
pub fn is_overloaded(total: u32) -> bool {
    total >= CAPACITY
}
