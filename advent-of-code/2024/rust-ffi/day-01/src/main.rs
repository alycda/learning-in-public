//! 2024 Day 1: Historian Hysteria
//!
//! see the README for more info

use day_01::{
    process_part1, process_part2, process_part1_c, process_part2_c,
    process_part2_bsearch, process_part2_freqmap,
    process_part2_glibc_count, process_part2_glibc_freqmap,
    process_part1_libc, process_part2_libc,
    process_part2_glib, process_part2_uthash,
    SAMPLE_INPUT
};

fn main() {
    println!("2024 Day 1");

    assert_eq!(process_part1("3 7").unwrap(), process_part1_c("3 7").unwrap());
    assert_eq!(process_part1_c("9 3").unwrap(), process_part1("9 3").unwrap());

    let part1_rust = process_part1(SAMPLE_INPUT).unwrap();
    let part1_c = process_part1_c(SAMPLE_INPUT).unwrap();
    let part1_libc = process_part1_libc(SAMPLE_INPUT).unwrap();

    println!("Part 1:");
    println!("  Rust:            {}", part1_rust);
    println!("  Manual FFI:      {}", part1_c);
    println!("  libc crate:      {}", part1_libc);

    let part2 = process_part2(SAMPLE_INPUT).unwrap();
    let part2_c = process_part2_c(SAMPLE_INPUT).unwrap();
    let part2_bsearch = process_part2_bsearch(SAMPLE_INPUT).unwrap();
    let part2_freqmap = process_part2_freqmap(SAMPLE_INPUT).unwrap();
    let part2_glibc_count = process_part2_glibc_count(SAMPLE_INPUT).unwrap();
    let part2_glibc_freqmap = process_part2_glibc_freqmap(SAMPLE_INPUT).unwrap();
    let part2_libc = process_part2_libc(SAMPLE_INPUT).unwrap();
    let part2_glib = process_part2_glib(SAMPLE_INPUT).unwrap();
    let part2_uthash = process_part2_uthash(SAMPLE_INPUT).unwrap();

    assert_eq!(part2, 31);
    assert_eq!(part2_c, 31);
    assert_eq!(part2_bsearch, 31);
    assert_eq!(part2_freqmap, 31);
    assert_eq!(part2_glibc_count, 31);
    assert_eq!(part2_glibc_freqmap, 31);
    assert_eq!(part2_libc, 31);
    assert_eq!(part2_glib, 31);
    assert_eq!(part2_uthash, 31);

    println!("\nPart 2:");
    println!("  Rust:            {}", part2);
    println!("  C-style:         {}", part2_c);
    println!("  Binary search:   {}", part2_bsearch);
    println!("  FreqMap (sim):   {}", part2_freqmap);
    println!("  Real C count:    {}", part2_glibc_count);
    println!("  Real C freqmap:  {}", part2_glibc_freqmap);
    println!("  libc crate:      {}", part2_libc);
    println!("  GLib hash table: {}", part2_glib);
    println!("  uthash:          {}", part2_uthash);
}
