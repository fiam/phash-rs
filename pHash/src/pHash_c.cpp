#include "pHash.h"

// Wrapper functions in exporting an exception-safe API

extern "C" {
    
int ph_dct_imagehash_c(const char *filename, ulong64 *hash);
uint8_t *ph_mh_imagehash_c(const char *filename, int &N, float alpha,
                           float lvl);
}

int ph_dct_imagehash_c(const char *filename, ulong64 *hash) {
    try {
        return ph_dct_imagehash(filename, *hash);
    } catch (std::exception &ex) {
        printf("%s: caught exception: %s", __func__, ex.what());
    }
    return -1;
}

uint8_t *ph_mh_imagehash_c(const char *filename, int &N, float alpha,
                           float lvl) {
    try {
        return ph_mh_imagehash(filename, N, alpha, lvl);
    } catch (std::exception &ex) {
        printf("%s: caught exception: %s", __func__, ex.what());
    }
    return nullptr;
}
