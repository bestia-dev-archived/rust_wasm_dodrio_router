// region: lmake_readme include "readme.md" //! A
//! # rust_wasm_dodrio_router
//! 
//! version: 0.4.6  date: 2020-05-09 authors: Luciano Bestia  
//! **wasm router for local hash routes for dodrio vdom**
//! 
//! 
//! ## local router with hash for dodrio vdom
//! 
//! I needed a router for local hash routes in rust wasm for dodrio vdom.  
//! This library contains the generic parts.  
//! 
//! In the project add a file/mod with specific implementation code like this example:  
//! <https://github.com/LucianoBestia/mem6_game/blob/master/mem6/src/routerimplmod.rs>  
//! 
//! ## cargo crev reviews and advisory
//! 
//! It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
//! to verify the trustworthiness of each of your dependencies.  
//! Please, spread this info.  
//! On the web use this url to read crate reviews. Example:  
//! <https://bestia.dev/cargo_crev_web/query/num-traits>  
// endregion: lmake_readme include "readme.md" //! A

pub mod router_mod;

// use rust_wasm_websys_utils::*;
