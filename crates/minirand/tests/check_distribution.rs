mod helpers;
use helpers::{write_csv, write_excel_xml};

const COMBINED_SHEET_PATH: &str = "./tests/outputs/combined-outputs.xml";

fn clean(path: &str) {
    use std::fs;
    let _ = fs::remove_file(path);
}


#[test]
fn test_u8() {
    use minirand::RandomU8;
    fn run()  -> [u16; 256] {
        let mut distribution: [u16; 256] = [0; 256];
        // let max: u128 = u128::MAX;
        for _ in u16::MIN..u16::MAX
        {
            let number = RandomU8::random_num(0, 255);
            distribution[number as usize] += 1;
        }
        distribution
        // println!("Term: Count");
        // for i in u8::MIN..u8::MAX {
        //     println!("{}: {}", i, distribution[i as usize]);
        // }
    }
    const NUMBER_OF_DISTRIBUTIONS: usize = 50;
    let mut all_distributions: [[u16; 256]; NUMBER_OF_DISTRIBUTIONS] = [[0; 256]; NUMBER_OF_DISTRIBUTIONS];
    let mut average: [u64; 256] = [0; 256];
    
    for i in 0..NUMBER_OF_DISTRIBUTIONS {
        all_distributions[i] = run();
    }
    for x in 0..256 {
        for i in 0..NUMBER_OF_DISTRIBUTIONS {
            average[x] += all_distributions[i][x] as u64;
        }
    }
    for x in 0..256 {
        average[x] /= NUMBER_OF_DISTRIBUTIONS as u64;
    }
    println!("Term: Count");
    for i in 0..256 {
        println!("{}: {}", i,average[i]);
    }
    let path = "./tests/outputs/u8.csv";
    clean(path);
    
    let _ = write_csv(path, &average).expect("Failed to write csv");
    
    let sheet_name: &str = "u8";
    let _ = write_excel_xml(COMBINED_SHEET_PATH, sheet_name, &average);
    // let mut distribution: Vec<u128> = Vec::new();
    
}


#[test]
fn test_f32_distribution() {
    use minirand::RandomF32;
    const NUM_BUCKETS: usize = 256;               // number of histogram buckets
    const SAMPLES_PER_RUN: usize = 65535;         // match u16 iteration count
    const NUMBER_OF_DISTRIBUTIONS: usize = 50;

    fn run() -> [f32; NUM_BUCKETS] {
        let mut distribution = [0f32; NUM_BUCKETS];

        for _ in 0..SAMPLES_PER_RUN {
            let value = RandomF32::random_num(0.0, 1.0); 
            
            // clamp to avoid 1.0 going out of bounds
            let index = (value * NUM_BUCKETS as f32)
                .floor()
                .clamp(0.0, (NUM_BUCKETS - 1) as f32) as usize;

            distribution[index] += 1.0;
        }

        distribution
    }

    let mut all_distributions = [[0f32; NUM_BUCKETS]; NUMBER_OF_DISTRIBUTIONS];
    let mut average = [0f64; NUM_BUCKETS];

    // run multiple batches
    for i in 0..NUMBER_OF_DISTRIBUTIONS {
        all_distributions[i] = run();
    }

    // accumulate
    for bucket in 0..NUM_BUCKETS {
        for i in 0..NUMBER_OF_DISTRIBUTIONS {
            average[bucket] += all_distributions[i][bucket] as f64;
        }
    }

    // compute mean
    for bucket in 0..NUM_BUCKETS {
        average[bucket] /= NUMBER_OF_DISTRIBUTIONS as f64;
    }

    println!("Bucket: Count");
    for i in 0..NUM_BUCKETS {
        println!("{}: {}", i, average[i]);
    }

    let path = "./tests/outputs/f32_distribution.csv";
    clean(path);
    let _ = write_csv(path, &average).expect("Failed to write csv");
    // path: &str,
    // sheet_name: &str,
    // data: &[T; DISTRIBUTION_LENGTH]


    let sheet_name: &str = "f32";
    let _ = write_excel_xml(COMBINED_SHEET_PATH, sheet_name, &average);

}
