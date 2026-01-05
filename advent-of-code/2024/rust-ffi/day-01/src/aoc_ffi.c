#include "aoc_ffi.h"
#include <stdlib.h>
#include <string.h>

// Linear scan to count occurrences
size_t count_occurrences(const int32_t *arr, size_t len, int32_t target) {
    size_t count = 0;
    for (size_t i = 0; i < len; i++) {
        if (arr[i] == target) {
            count++;
        }
    }
    return count;
}

// Build frequency map from array
FreqMap* freqmap_build(const int32_t *arr, size_t len) {
    if (len == 0 || arr == NULL) {
        return NULL;
    }

    // Allocate the FreqMap structure
    FreqMap *map = (FreqMap*)malloc(sizeof(FreqMap));
    if (map == NULL) {
        return NULL;
    }

    // Find min and max values
    int32_t min_val = arr[0];
    int32_t max_val = arr[0];
    for (size_t i = 1; i < len; i++) {
        if (arr[i] < min_val) min_val = arr[i];
        if (arr[i] > max_val) max_val = arr[i];
    }

    map->min_val = min_val;
    map->max_val = max_val;

    // Allocate and zero the counts array
    size_t range = (size_t)(max_val - min_val + 1);
    map->counts = (int32_t*)calloc(range, sizeof(int32_t));
    if (map->counts == NULL) {
        free(map);
        return NULL;
    }

    // Count frequencies
    for (size_t i = 0; i < len; i++) {
        size_t idx = (size_t)(arr[i] - min_val);
        map->counts[idx]++;
    }

    return map;
}

// Lookup count in frequency map
int32_t freqmap_get(const FreqMap *map, int32_t value) {
    if (map == NULL || map->counts == NULL) {
        return 0;
    }

    if (value < map->min_val || value > map->max_val) {
        return 0;
    }

    size_t idx = (size_t)(value - map->min_val);
    return map->counts[idx];
}

// Free frequency map
void freqmap_free(FreqMap *map) {
    if (map != NULL) {
        if (map->counts != NULL) {
            free(map->counts);
            map->counts = NULL;
        }
        free(map);
    }
}
