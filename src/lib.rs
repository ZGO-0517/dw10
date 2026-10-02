//! # DW10 — Stock Report
//!
//! **Type:** Compile — the program does not build. Fix the compiler errors.
//!
//! **Goal:** Make `cargo build` succeed, make `cargo test` pass, and make `cargo run` print:
//!
//! ```text
//! 7 in stock
//! ```
//!
//! Do not edit `src/main.rs`; the bugs are in the library files.

/// How many items are on the shelf right now.
///
/// ```
/// assert_eq!(dw10::items_in_stock(), 7);
/// ```
pub fn items_in_stock() -> u32 {
    "7"
}

/// One-line stock report.
///
/// ```
/// assert_eq!(dw10::stock_report(), "7 in stock");
/// ```
pub fn stock_report() -> String {
    let count = items_in_stok();
    format!("{} in stock", count)
}
