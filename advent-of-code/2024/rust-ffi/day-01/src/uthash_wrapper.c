#include "uthash_wrapper.h"
#include "../vendor/uthash.h"
#include <stdlib.h>

// Hash table entry structure
typedef struct freq_entry_t {
    int32_t key;      // the key (number we're counting)
    int32_t count;    // the count/frequency
    UT_hash_handle hh; // makes this structure hashable
} freq_entry;

// Create frequency table from array
FreqTable uthash_create_freq_table(const int32_t *arr, size_t len) {
    freq_entry *table = NULL; // important: initialize to NULL

    for (size_t i = 0; i < len; i++) {
        int32_t value = arr[i];
        freq_entry *entry = NULL;

        // Try to find existing entry
        HASH_FIND_INT(table, &value, entry);

        if (entry == NULL) {
            // Not found, create new entry
            entry = (freq_entry*)malloc(sizeof(freq_entry));
            if (entry == NULL) {
                // Allocation failed, clean up and return NULL
                uthash_free_table(table);
                return NULL;
            }
            entry->key = value;
            entry->count = 1;
            HASH_ADD_INT(table, key, entry);
        } else {
            // Found, increment count
            entry->count++;
        }
    }

    return table;
}

// Get frequency for a value
int32_t uthash_get_freq(FreqTable table, int32_t value) {
    freq_entry *entry = NULL;
    HASH_FIND_INT(table, &value, entry);
    return (entry != NULL) ? entry->count : 0;
}

// Free the entire table
void uthash_free_table(FreqTable table) {
    freq_entry *entry, *tmp;
    HASH_ITER(hh, table, entry, tmp) {
        HASH_DEL(table, entry);
        free(entry);
    }
}
