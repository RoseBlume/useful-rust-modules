use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let release_build = !cfg!(debug_assertions);

        // =======================================================================
    // SELECT ASM FILES PER ARCHITECTURE
    // =======================================================================

    // ============================================
    // READ REAL TARGET (NOT HOST)
    // ============================================

    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let asm_files: Vec<&str> = match target_os.as_str() {
        "windows" => match target_arch.as_str() {
            "x86" => vec![
                "asm/x86/windows/iseven.asm",
                "asm/x86/windows/print.asm",
                "asm/x86/windows/qsqrt.asm",
            ],

            "x86_64" => vec![
                "asm/x86_64/windows/iseven.asm",
                "asm/x86_64/windows/print.asm",
                "asm/x86_64/windows/qsqrt.asm",
            ],

            "aarch64" => vec![
                "asm/aarch64/windows/iseven.asm",
                "asm/aarch64/windows/print.asm",
                "asm/aarch64/windows/qsqrt.asm",
            ],

            other => panic!("Unsupported target architecture: {}", other),
        },
        "linux" => match target_arch.as_str() {
            "x86" => vec![
                "asm/x86/linux/iseven.asm",
                "asm/x86/linux/print.asm",
                "asm/x86/linux/qsqrt.asm",
            ],

            "x86_64" => vec![
                "asm/x86_64/linux/iseven.asm",
                "asm/x86_64/linux/print.asm",
                "asm/x86_64/linux/qsqrt.asm",
            ],

            "aarch64" => vec![
                "asm/aarch64/linux/iseven.asm",
                "asm/aarch64/linux/print.asm",
                "asm/aarch64/linux/qsqrt.asm",
            ],

            other => panic!("Unsupported target architecture: {}", other),
        },
        other => panic!("Unsupported OS: {}", other),
    };


    // ============================================
    // SELECT ASSEMBLER BASED ON TARGET (NOT HOST)
    // ============================================

    println!("cargo:rustc-env=CC_NO_DEFAULTS=1");

    match target_arch.as_str() {

        "x86_64" => println!(
            "cargo:rustc-env=AS={}",
            r"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.44.35207\bin\Hostx64\x64\ml64.exe"
        ),

        "x86" => println!(
            "cargo:rustc-env=AS={}",
            r"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.44.35207\bin\Hostx64\x86\ml.exe"
        ),

        "aarch64" => println!(
            "cargo:rustc-env=AS={}",
            r"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.44.35207\bin\Hostx64\arm64\ml64.exe"
        ),

        _ => {}
    }

    let m_value = match target_arch.as_str() {
        "x86_64" | "aarch64" => "-m64",
        "x86" => "-m32",
        other => panic!("Unsupported Architecture: {}", other)
    };
    for file in &asm_files {
        println!("{}", file);
    }

    // --- END FIXES ---

    let mut build = cc::Build::new();
    build
        .target(&env::var("TARGET").unwrap())
        .out_dir(&out_dir)
        .files(&asm_files)
        .cargo_metadata(true);

    if release_build {
        build.flag("/O1");
        build.flag("/GL");
    }
    build.flag("/c");
    build.flag("/coff");
    build.flag("/Cp");
    // /c /coff /Cp
    build.flag(m_value);
    build.compile("asm_lib");

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=asm_lib");

    if release_build {
        println!("cargo:rustc-link-arg=/OPT:REF");
        println!("cargo:rustc-link-arg=/OPT:ICF");
    }
}
