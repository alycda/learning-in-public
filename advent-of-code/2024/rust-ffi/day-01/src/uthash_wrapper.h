#ifndef UTHASH_WRAPPER_H
#define UTHASH_WRAPPER_H

#include <stdint.h>
#include <stddef.h>

// Opaque pointer to uthash table
typedef struct freq_entry_t* FreqTable;

// Create a new frequency table
FreqTable uthash_create_freq_table(const int32_t *arr, size_t len);

// Get frequency count for a value
int32_t uthash_get_freq(FreqTable table, int32_t value);

// Free the frequency table
void uthash_free_table(FreqTable table);

#endif // UTHASH_WRAPPER_H
