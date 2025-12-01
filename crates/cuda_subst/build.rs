fn main() {
    let nvcc = r"C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.0\bin\nvcc.exe";

    cc::Build::new()
        .cuda(true)
        .no_default_flags(true)
        .warnings(false)
        .compiler(nvcc)
        .file("cuda/brute_force.cu")
        .flag("-O3")
        .flag("-Xcompiler")
        .flag("/MD")
        .flag("-arch=sm_75") // adjust for your GPU
        .compile("cuda_bruteforce");

    println!("cargo:rustc-link-lib=cudart");
    println!("cargo:rustc-link-search=native=C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v13.0\\lib\\x64");
    println!("cargo:rerun-if-changed=cuda/brute_force.cu");
}
