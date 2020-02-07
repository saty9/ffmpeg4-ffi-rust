# ffmpeg4-ffi

Rust bindings for ffmpeg, fork of ffmpeg-dev

Differences from original repo:
- Dynamically-linked against ffmpeg libs
- Bindgen performs using system-installed headers
- Generated bindings are stored in repo without use of `include!` macro

This means:
- Resulting application will require ffmpeg 4.x libs installed
- Bindings re-generation will require ffmpeg headers installed
  which are only shipped in `-dev` packages in some distros.
- Putting bindings right into source tree fixes issues with
  some IDE's unable to resolve `include!` macro

This crate is intended to be a drop-in replacement for ffmpeg-dev,
so you (probably) can just use it instead of original if you need dynamic linking.

If you're looking for self-contained, statically-linked bindigs, 
take a look at original repo: https://github.com/imager-io/ffmpeg-dev-rs

## License

- This repo, as well as the original, is licensed under MIT.
- FFmpeg itself is licensed under LGPL

