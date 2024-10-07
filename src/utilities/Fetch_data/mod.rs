mod fetch_depth;
mod fetch_earnings;
mod fetch_RUNEPool;
mod fetch_swaps;

pub use fetch_depth::{fetch_depth_main};
pub use fetch_earnings::{fetch_earnings_main};
pub use fetch_RUNEPool::{fetch_runepool_main};
pub use fetch_swaps::{fetch_swaps_main};