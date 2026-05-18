use std::path::PathBuf;

fn main() {
    let cpp_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../MCPQueueApp");

    cc::Build::new()
        .cpp(true)
        .flag_if_supported("/std:c++17")
        .flag_if_supported("-std=c++17")
        .file(cpp_dir.join("queue_ffi.cpp"))
        .include(&cpp_dir)
        .compile("queue_cpp");

    println!("cargo:rerun-if-changed=../MCPQueueApp/Queue.h");
    println!("cargo:rerun-if-changed=../MCPQueueApp/queue_ffi.h");
    println!("cargo:rerun-if-changed=../MCPQueueApp/queue_ffi.cpp");
}
