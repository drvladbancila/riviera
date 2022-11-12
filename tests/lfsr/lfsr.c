#include "lfsr.h"
#define START_STATE 0xF3AD

unsigned int lfsr(void) {
    static unsigned int lfsr = START_STATE;
    static unsigned int bit;

    bit = ((lfsr >> 0) ^ (lfsr >> 2) ^ (lfsr >> 3) ^ (lfsr >> 5)) & 1u;
    lfsr = (lfsr >> 1) | (bit << 15);

    return lfsr;
}