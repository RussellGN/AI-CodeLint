#!/bin/bash

DIR="4. edge_case"
mkdir -p "$DIR"

cat > "$DIR/p01_float_equality.py" << 'EOF'
def is_equal(a: float, b: float) -> bool:
    return a - b == 0
EOF

cat > "$DIR/p02_recursive_default.py" << 'EOF'
def build_tree(val, children=[]):
    return {"val": val, "children": children}
EOF

cat > "$DIR/p03_iterator_exhaustion.py" << 'EOF'
def process(nums):
    gen = (x * 2 for x in nums)
    total = sum(gen)
    items = list(gen)
    return total, items
EOF

cat > "$DIR/t01_nan_comparison.ts" << 'EOF'
function isInvalid(value: number): boolean {
    return value === NaN;
}
EOF

cat > "$DIR/t02_object_spread_mutation.ts" << 'EOF'
function updateConfig(base: object, patch: object): object {
    return Object.assign(base, patch);
}
EOF

cat > "$DIR/t03_precision_loss.ts" << 'EOF'
function toLargeInt(value: string): number {
    return parseInt(value);
}

const id = toLargeInt("9007199254740993");
EOF

cat > "$DIR/c01_buffer_off_by_one.c" << 'EOF'
void copy_string(char *dst, const char *src, int max_len) {
    for (int i = 0; i < max_len; i++) {
        dst[i] = src[i];
    }
}
EOF

cat > "$DIR/c02_int_overflow.c" << 'EOF'
int midpoint(int lo, int hi) {
    return (lo + hi) / 2;
}
EOF

cat > "$DIR/c03_strtol_unchecked.c" << 'EOF'
#include <stdlib.h>
long parse(const char *s) {
    return strtol(s, NULL, 10);
}
EOF

cat > "$DIR/r01_cast_truncation.rs" << 'EOF'
fn to_u8(val: u32) -> u8 {
    val as u8
}
EOF

cat > "$DIR/r02_float_loop.rs" << 'EOF'
fn count_steps(step: f64) -> u32 {
    let mut x = 0.0;
    let mut count = 0;
    while x != 1.0 {
        x += step;
        count += 1;
    }
    count
}
EOF

cat > "$DIR/r03_unwrap_in_loop.rs" << 'EOF'
fn first_words(lines: &[&str]) -> Vec<&str> {
    lines.iter().map(|l| l.split_whitespace().next().unwrap()).collect()
}
EOF

cat > "$DIR/j01_integer_overflow.java" << 'EOF'
public class Overflow {
    public static long factorial(int n) {
        int result = 1;
        for (int i = 2; i <= n; i++) {
            result *= i;
        }
        return result;
    }
}
EOF

cat > "$DIR/j02_equals_contract.java" << 'EOF'
public class Point {
    int x, y;
    public boolean equals(Point other) {
        return this.x == other.x && this.y == other.y;
    }
}
EOF

cat > "$DIR/j03_thread_visibility.java" << 'EOF'
public class Flag {
    private boolean running = true;

    public void stop() { running = false; }

    public void run() {
        while (running) {}
    }
}
EOF

echo "Done. $(ls "$DIR" | wc -l) files created in '$DIR'."