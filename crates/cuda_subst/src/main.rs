use rand::seq::SliceRandom;
use rand::rng;
use cuda_subst::launch_score_kernel;

fn random_key() -> [u8; 26] {
    let mut arr = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    arr.shuffle(&mut rng());
    arr
}

fn main() {
    let ciphertext = b"ZICVTWQNGRZGVTWAVZHCQYGLMGJ";
    let num_keys = 50000;

    let mut host_keys = vec![0u8; num_keys * 26];
    for i in 0..num_keys {
        let key = random_key();
        host_keys[i * 26..(i + 1) * 26].copy_from_slice(&key);
    }

    let mut scores = vec![0f32; num_keys];

    unsafe {
        launch_score_kernel(
            ciphertext.as_ptr() as *const i8,
            ciphertext.len() as i32,
            host_keys.as_ptr() as *const i8,
            num_keys as i32,
            scores.as_mut_ptr(),
        );
    }

    let (best_idx, best_score) = scores
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .unwrap();

    println!("Best key index: {best_idx}, score: {best_score}");
    print!("Key: ");
    for ch in &host_keys[best_idx * 26..(best_idx + 1) * 26] {
        print!("{}", *ch as char);
    }
    println!();
}
