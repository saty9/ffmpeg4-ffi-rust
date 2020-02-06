//! Links:
//! * [FFmpeg docs overview](https://ffmpeg.org/documentation.html)
//! * [FFmpeg C API documentation](https://ffmpeg.org/doxygen/trunk/index.html)
//! * [Rust docs](https://docs.rs/ffmpeg-dev)
//! * [C Examples](https://github.com/FFmpeg/FFmpeg/tree/master/doc/examples) (Pretty easy to convert to rust in my experience.)
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(improper_ctypes)]
#![allow(safe_packed_borrows)]

pub mod sys;
pub mod api;
pub mod extra;