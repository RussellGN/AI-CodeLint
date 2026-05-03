void copy_string(char *dst, const char *src, int max_len) {
    for (int i = 0; i < max_len; i++) {
        dst[i] = src[i];
    }
}
