use std::fmt::Debug;

/// Print an entire array. Creates a new line at every `split_at` elements.
pub fn debug_array<I>(slice: I, title: &str, split_at: usize)
where
    I: IntoIterator,
    I::Item: Debug,
{
    println!("===== {title} =====\n");

    for (idx, num) in slice.into_iter().enumerate() {
        let line = idx / (split_at + 1);

        print!("a[{idx: >5}]={: <2} ", format!("{num:?}"));

        if idx == (split_at * (line + 1) + line) {
            println!();
        }
    }

    println!("\n\n");
}