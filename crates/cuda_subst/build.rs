fn main() {
    // Compile CUDA code
    let nvcc = "C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v13.0\\bin\\nvcc.exe";
    let mut build = cc::Build::new();
    build
        .cuda(true)
        .no_default_flags(true)        // <-- IMPORTANT
        .compiler(nvcc)
        .warnings(false)
        .file("cuda/score_keys.cu")
        .flag("-O3")
        .flag("-Xcompiler")
        .flag("/MD")                   // MSVC-compatible host compiler flag
        .flag("-arch=sm_75")           // adjust for your hardware
        .compile("cuda_score_keys");

    println!("cargo:rustc-link-lib=cudart");
    println!("cargo:rustc-link-search=native=C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v13.0\\lib\\x64");
    println!("cargo:rerun-if-changed=cuda/score_keys.cu");
}
