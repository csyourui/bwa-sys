use std::{env, path::PathBuf};

// make -C bwa-sys/bwa/ -n libbwa.a | grep -o -E "[A-Za-z0-9_]+\.c"
const FILES: &[&str] = &[
    "bwa/QSufSort.c",
    "bwa/bntseq.c",
    "bwa/bwa.c",
    "bwa/bwamem.c",
    "bwa/bwamem_extra.c",
    "bwa/bwamem_pair.c",
    "bwa/bwt.c",
    "bwa/bwt_gen.c",
    "bwa/bwtindex.c",
    "bwa/is.c",
    "bwa/kstring.c",
    "bwa/ksw.c",
    "bwa/kthread.c",
    "bwa/malloc_wrap.c",
    "bwa/rle.c",
    "bwa/rope.c",
    "bwa/utils.c",
];

// make -C bwa-sys/bwa/ -nd libbwa.a | grep -o -E "[A-Za-z0-9_]+\.h" | sort | uniq
const HEADERS: &[&str] = &[
    "bwa/QSufSort.h",
    "bwa/bntseq.h",
    "bwa/bwa.h",
    "bwa/bwamem.h",
    "bwa/bwt.h",
    "bwa/kbtree.h",
    "bwa/khash.h",
    "bwa/kseq.h",
    "bwa/ksort.h",
    "bwa/kstring.h",
    "bwa/ksw.h",
    "bwa/kvec.h",
    "bwa/malloc_wrap.h",
    "bwa/neon_sse.h",
    "bwa/rle.h",
    "bwa/rope.h",
    "bwa/scalar_sse.h",
    "bwa/utils.h",
];

fn main() {
    for file in FILES {
        println!("cargo:rerun-if-changed={}", file);
    }
    for file in HEADERS {
        println!("cargo:rerun-if-changed={}", file);
    }
    cc::Build::new()
        .define("COMPILATION_TIME_PLACE", "\"build.rs\"")
        .warnings(false)
        .extra_warnings(false)
        .files(FILES)
        .flag("-fPIC")
        .compile("bwa");

    // 告诉 cargo 链接外部库
    println!("cargo:rustc-link-lib=m");       // 链接 libm (数学库)
    println!("cargo:rustc-link-lib=z");       // 链接 libz (zlib 压缩库)
    println!("cargo:rustc-link-lib=pthread"); // 链接 libpthread (多线程库)
    
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // 自定义输出路径，将其直接输出到 src 目录
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
