#include "../shishua/shishua.h"

prng_state* shishua_bindings_init(uint64_t* seed) {
    prng_state* state = malloc(sizeof(prng_state));
    prng_init(state, seed);
    return state;
}

void shishua_bindings_destroy(prng_state* state) {
    free(state);
}

void shishua_bindings_generate(prng_state* state, uint8_t* buffer, size_t size) {
    prng_gen(state, buffer, size);
}
