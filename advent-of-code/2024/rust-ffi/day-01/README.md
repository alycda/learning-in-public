### Why qsort needs a comparison function:

1. **C doesn't have generics or type information at runtime** - `qsort` receives a `void*` (just raw bytes). It has no idea what those bytes mean. Are they integers? Floats? Strings? Structs?
1. **Different types need different comparison logic**:
- `int`: arithmetic comparison (`a - b`)
- `float`: can't use subtraction (NaN, precision issues)
- `char*`: need `strcmp`, not pointer comparison
-Structs: which field(s) to compare?
1. **The bytes themselves aren't sortable** - Without knowing the type, you can't just compare raw bytes:
- For `i32`: the value `256` is `[0, 1, 0, 0]` in little-endian bytes
- For `i32`: the value `1` is `[1, 0, 0, 0]` in little-endian bytes
- Byte-wise comparison would say `1 > 256` (because `[1...] > [0...]`) which is wrong!
1. **Endianness matters** - Even if you tried byte comparison, multi-byte integers are stored differently on different architectures.

The comparison function bridges the gap:
- You (the programmer) know the type
- You cast `void*` back to the actual type
- You provide the semantic comparison logic for that type

This is why Rust's `.sort(`) is so much nicer - it uses generics and the `Ord` trait, so the compiler generates the right comparison code automatically. C's `qsort` makes you do this manually, which is more error-prone but also shows you exactly what's happening under the hood!