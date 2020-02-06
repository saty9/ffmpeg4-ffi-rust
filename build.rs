use std::iter::FromIterator;
use std::collections::HashSet;
use std::path::PathBuf;
use std::env;

///////////////////////////////////////////////////////////////////////////////
// UTILS - ENVIROMENT
///////////////////////////////////////////////////////////////////////////////

#[allow(unused)]
fn out_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR env var"))
}

#[allow(unused)]
fn is_release_mode() -> bool {
    has_env_var_with_value("PROFILE", "release")
}

#[allow(unused)]
fn is_debug_mode() -> bool {
    has_env_var_with_value("PROFILE", "debug")
}

#[allow(unused)]
fn opt_level_eq(x: u8) -> bool {
    has_env_var_with_value("OPT_LEVEL", &format!("{}", x))
}

fn has_env_var_with_value(s: &str, v: &str) -> bool {
    std::env::var(s)
        .map(|x| x.as_str() == v)
        .unwrap_or(false)
}

///////////////////////////////////////////////////////////////////////////////
// PATHS
///////////////////////////////////////////////////////////////////////////////

pub const LIBS: &[&str] = &[
    "avcodec",
    "avdevice",
    "avfilter",
    "avformat",
    "avutil",
    "swresample",
    "swscale"
];

///////////////////////////////////////////////////////////////////////////////
// CODEGEN
///////////////////////////////////////////////////////////////////////////////

// See https://github.com/rust-lang/rust-bindgen/issues/687#issuecomment-450750547
#[derive(Debug, Clone)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// BUILD PIPELINE
///////////////////////////////////////////////////////////////////////////////

fn build() {
    // LINK
    for name in LIBS {
        println!("cargo:rustc-link-lib=dylib={}", name);
    }
    // CODEGEN
    {
        let gen_file_name = "src/sys.rs";
        let ignored_macros = IgnoreMacros(HashSet::from_iter(vec![
            String::from("FP_INFINITE"),
            String::from("FP_NAN"),
            String::from("FP_NORMAL"),
            String::from("FP_SUBNORMAL"),
            String::from("FP_ZERO"),
            String::from("IPPORT_RESERVED"),
        ]));

        if has_env_var_with_value("FF_DO_CODEGEN", "1") {
            // RUN
            bindgen::Builder::default()
                .header("headers.h")
                .parse_callbacks(Box::new(ignored_macros.clone()))
                .layout_tests(false)
                .rustfmt_bindings(true)
                .detect_include_paths(true)
                .generate_comments(true)
                .generate()
                .expect("Unable to generate bindings")
                .write_to_file(gen_file_name)
                .expect("Couldn't write bindings!");
        }
    }
    // COMPILE CBITS
    cc::Build::new()
        .file("cbits/defs.c")
        .file("cbits/img_utils.c")
        .compile("cbits");
}

///////////////////////////////////////////////////////////////////////////////
// MAIN
///////////////////////////////////////////////////////////////////////////////

fn main() {
    build();
}
