typedef unsigned int u32;

u32 mul(u32 a, u32 b) {
    u32 i;
    u32 r = 0;

    for (i = b; i > 0; i--) {
        r += a;
    }

    return r;
}