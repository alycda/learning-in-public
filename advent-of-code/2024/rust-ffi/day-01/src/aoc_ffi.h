#ifndef AOC_FFI_H
#define AOC_FFI_H

#include <stddef.h>
#include <stdint.h>

// Count occurrences of target in array
// Returns the number of times target appears in arr
size_t count_occurrences(const int32_t *arr, size_t len, int32_t target);

// Frequency map structure
typedef struct {
    int32_t min_val;
    int32_t max_val;
    int32_t *counts;
} FreqMap;

// Build a frequency map from an array
// Returns an allocated FreqMap that must be freed with freqmap_free
FreqMap* freqmap_build(const int32_t *arr, size_t len);

// Get count for a value from frequency map
// Returns 0 if value not in map
int32_t freqmap_get(const FreqMap *map, int32_t value);

// Free a frequency map
void freqmap_free(FreqMap *map);

#endif // AOC_FFI_H
