//! Reactive/signals-based API for Blaeck.
//!
//! This is the **recommended way** to build interactive terminal UIs with Blaeck.
//! It provides a React-like hooks system with automatic re-rendering when state changes.
//!
//! # Quick Start
//!
//! ```ignore
//! use blaeck::reactive::*;
//! use blaeck::prelude::*;
//!
//! fn counter(cx: Scope) -> Element {
//!     // Create reactive state
//!     let count = use_state(cx.clone(), || 0);
//!
//!     // Clone signal for use in input handler
//!     let count_handler = count.clone();
//!     use_input(cx, move |key| {
//!         if key.is_char(' ') {
//!             count_handler.set(count_handler.get() + 1);
//!         }
//!     });
//!
//!     // UI automatically re-renders when count changes
//!     element! {
//!         Box(border_style: BorderStyle::Round, padding: 1.0) {
//!             Text(content: format!("Count: {}", count.get()))
//!         }
//!     }
//! }
//!
//! fn main() -> std::io::Result<()> {
//!     ReactiveApp::run(counter)
//! }
//! ```
//!
//! # Why Reactive?
//!
//! The reactive API provides several advantages over manual state management:
//!
//! - **Automatic re-rendering**: Call `signal.set()` and the UI updates
//! - **Declarative**: State and UI are connected, not manually synchronized
//! - **Familiar**: Similar to React, Solid.js, and other modern UI frameworks
//! - **Testable**: Multiple ReactiveApp instances can coexist (no global state)
//!
//! # Available Hooks
//!
//! | Hook | Purpose |
//! |------|---------|
//! | [`use_state`] | Create reactive state that triggers re-render on change |
//! | [`use_input`] | Register keyboard input handler (runs once, persists across renders) |
//! | [`use_timeline`] | Create a declarative animation timeline with playback controls |
//!
//! Future hooks (v0.3.0+): `use_effect`, `use_memo`, `use_const`
//!
//! # Rules of Hooks
//!
//! Like React, hooks must follow certain rules:
//!
//! 1. **Call hooks unconditionally** - Don't put hooks inside `if` or `match`
//! 2. **Same order every render** - Hooks are identified by call order
//! 3. **Only in components** - Hooks only work inside component functions
//!
//! ```ignore
//! // WRONG - conditional hook
//! fn bad(cx: Scope) -> Element {
//!     if some_condition {
//!         let state = use_state(cx, || 0);  // Will panic on re-render!
//!     }
//!     // ...
//! }
//!
//! // CORRECT - unconditional hooks
//! fn good(cx: Scope) -> Element {
//!     let state = use_state(cx.clone(), || 0);  // Always called
//!     if some_condition {
//!         // Use the state here
//!     }
//!     // ...
//! }
//! ```
//!
//! # Clone Pattern for Closures
//!
//! Since `Scope` and `Signal` don't implement `Copy`, you need to clone them
//! before using in closures:
//!
//! ```ignore
//! fn my_component(cx: Scope) -> Element {
//!     let count = use_state(cx.clone(), || 0);  // Clone cx for use_state
//!     let name = use_state(cx.clone(), || String::new());  // Clone again
//!
//!     // Clone signals for the input handler
//!     let count_handler = count.clone();
//!     use_input(cx, move |key| {  // cx moved here (last use)
//!         count_handler.set(count_handler.get() + 1);
//!     });
//!
//!     // Original signals still usable for rendering
//!     element! {
//!         Text(content: format!("{}: {}", name.get(), count.get()))
//!     }
//! }
//! ```
//!
//! # Design Principles
//!
//! - **No thread-local state**: Runtime is explicit via [`RuntimeHandle`]
//! - **Hooks use cursor model**: Like React, hooks must be called in consistent order
//! - **`'static` closures**: Event handlers must own their data (use `use_state`)
//! - **Full rerender**: Any state change triggers complete re-render (no diffing in v0.2.0)
//!
//! # Important: Closure Lifetime Requirements
//!
//! Event handlers passed to `use_input` must be `'static`. This means you cannot
//! capture references to stack variables:
//!
//! ```ignore
//! // WON'T COMPILE - items is borrowed from stack
//! let items = vec!["A", "B"];
//! use_input(cx, move |key| { items.len(); });
//!
//! // CORRECT - use use_state for reactive data
//! let items = use_state(cx.clone(), || vec!["A", "B"]);
//! let items_handler = items.clone();
//! use_input(cx, move |key| { items_handler.get().len(); });
//! ```

mod app;
mod hooks;
mod instance;
mod runtime;
mod scope;
mod signal;

pub use app::{ReactiveApp, ReactiveAppConfig, ReactiveAppResult};
pub use hooks::{use_input, use_state, use_timeline, TimelineHandle};
pub use instance::{ComponentInstance, HookSlot};
pub use runtime::{ComponentId, RuntimeHandle, RuntimeInner};
pub use scope::Scope;
pub use signal::Signal;
