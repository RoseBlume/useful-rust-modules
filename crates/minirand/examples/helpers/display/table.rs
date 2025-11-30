pub fn build_tables(
    f32_vals: &[f32],
    f64_vals: &[f64],
    i8_vals: &[i8],
    i16_vals: &[i16],
    i32_vals: &[i32],
    i64_vals: &[i64],
    u8_vals: &[u8],
    u16_vals: &[u16],
    u32_vals: &[u32],
    u64_vals: &[u64],
) {

    // --- Random floats ---
    println!("--- Random floats ---");
    let headers = ["f32", "f64"];
    let widths = [15, 25];
    print_separator(&widths);
    print_header(&headers, &widths);
    print_separator(&widths);

    for (f32_val, f64_val) in f32_vals.iter().zip(f64_vals.iter()) {
        print_row(&[format!("{:e}", f32_val), format!("{:e}", f64_val)], &widths);
    }
    print_separator(&widths);

    // --- Random signed integers ---
    println!("\n--- Random signed integers ---");
    let headers = ["i8", "i16", "i32", "i64"];
    let widths = [6, 8, 13, 22];
    print_separator(&widths);
    print_header(&headers, &widths);
    print_separator(&widths);

    for ((i8_val, i16_val), (i32_val, i64_val)) in
        i8_vals.iter().zip(i16_vals.iter())
            .zip(i32_vals.iter().zip(i64_vals.iter()))
    {
        print_row(&[i8_val.to_string(), i16_val.to_string(), i32_val.to_string(), i64_val.to_string()], &widths);
    }
    print_separator(&widths);

    // --- Random unsigned integers ---
    println!("\n--- Random unsigned integers ---");
    let headers = ["u8", "u16", "u32", "u64"];
    let widths = [5, 7, 12, 22];
    print_separator(&widths);
    print_header(&headers, &widths);
    print_separator(&widths);

    for ((u8_val, u16_val), (u32_val, u64_val)) in
        u8_vals.iter().zip(u16_vals.iter())
            .zip(u32_vals.iter().zip(u64_vals.iter()))
    {
        print_row(&[u8_val.to_string(), u16_val.to_string(), u32_val.to_string(), u64_val.to_string()], &widths);
    }
    print_separator(&widths);
}

#[inline]
fn print_header(headers: &[&str], widths: &[usize]) {
    for (h, w) in headers.iter().zip(widths.iter()) {
        print!("|{:^width$}", h, width = *w);
    }
    println!("|");
}


#[inline]
fn print_row<T: std::fmt::Display>(values: &[T], widths: &[usize]) {
    for (v, w) in values.iter().zip(widths.iter()) {
        print!("| {:<width$} ", v, width = *w - 2); // 1 space on each side
    }
    println!("|");
}

#[inline]
fn print_separator(widths: &[usize]) {
    for w in widths {
        print!("+{}", "_".repeat(*w));
    }
    println!("+");
}