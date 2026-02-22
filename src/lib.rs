//! `RabbitMQ` Playground.

#![cfg_attr(
    not(debug_assertions),
    deny(warnings, missing_docs),
    deny(clippy::todo, clippy::unwrap_used)
)]
#![cfg_attr(
    not(any(test, debug_assertions)),
    deny(clippy::print_stdout, clippy::dbg_macro)
)]

#[cfg(test)]
mod tests;
