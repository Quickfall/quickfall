macro_rules! declare_info {
    ($expr: ident, $ind: literal, $err: literal) => {
        pub const $expr: (usize, &str) = ($ind, $err);
    };
}

// Optimizations infos

declare_info!(REDUCED_CALL, 0, "call will be optimized here");
declare_info!(
    SSA_VARIABLE,
    1,
    "variable will be treated as a register value"
);
declare_info!(
    LINEARIZATION,
    2,
    "function will be linearized as it is pure recursion"
);
