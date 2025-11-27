use std::process::{Command, Stdio};
use serde_json::Value;

fn main() {
    let features = load_features();
    let filtered = filter_features(&features);

    print_features(&filtered);

    for feature in filtered {
        check_feature(&feature);
    }

    println!("All features built successfully!");
}

fn load_features() -> Vec<String> {
    let output = Command::new("cargo")
        .args(&["metadata", "--no-deps", "--format-version=1"])
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to run cargo metadata");

    let metadata = String::from_utf8_lossy(&output.stdout);

    let v: Value = serde_json::from_str(&metadata)
        .expect("Failed to parse cargo metadata JSON");

    let features_obj = v["packages"][0]["features"]
        .as_object()
        .expect("Metadata features not an object");

    features_obj.keys().cloned().collect()
}

fn filter_features(features: &[String]) -> Vec<String> {
    features
        .iter()
        .filter(|f| *f != "test" && *f != "check-features")
        .cloned()
        .collect()
}

fn print_features(features: &[String]) {
    println!("Found {} usable features:", features.len());
    for f in features {
        println!(" - {}", f);
    }
    println!();
}

fn check_feature(feature: &str) {
    println!("===============================");
    println!("Checking feature: {}", feature);
    println!("===============================");

    let status = Command::new("cargo")
        .args(&[
            "build",
            "--no-default-features",
            "--features",
            feature,
        ])
        .status()
        .expect("Failed to run cargo build");

    if !status.success() {
        eprintln!("❌ Build failed for feature: {}", feature);
        std::process::exit(1);
    }

    println!("✔ Feature `{}` OK\n", feature);
}
