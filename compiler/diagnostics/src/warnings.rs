macro_rules! declare_warning {
    ($expr: ident, $ind: literal, $err: literal) => {
        pub const $expr: (usize, &str) = ($ind, $err);
    };
}

// Unused

declare_warning!(UNUSED_VAR, 0, "unused variable: {}");
declare_warning!(UNUSED_FUNCTION, 1, "unused function: {}");
declare_warning!(UNUSED_TYPE, 2, "unused type: {}");
declare_warning!(UNUSED_ARGUMENT, 3, "unused argument: {}");
declare_warning!(UNUSED_USE, 4, "unused `use` statement: {}");

// Support (CPU)
declare_warning!(CPU_SUPPORTED_TYPE, 5, "this type is unsupported by the CPU");
declare_warning!(
    CPU_SUPPORTED_OP,
    6,
    "this operation is unsupported by the CPU"
);

// Support (Quickfall)
declare_warning!(
    EXPERIMENTAL_FEAT,
    7,
    "this feature was marked as experimental"
);
declare_warning!(
    EXPERIMENTAL_FUNC,
    8,
    "this function was marked as experimental"
);
declare_warning!(EXPERIMENTAL_TYPE, 9, "this type was marked as experimental");
declare_warning!(DEPRECATED, 10, "this was marked as deprecated");
declare_warning!(DEPRECATED_MSG, 11, "this was marked as deprecated: {}");

// Safety
declare_warning!(UNSAFE_UNWRAP, 7, "unsafe unwrapping, value might not be {}");
declare_warning!(UNSAFE_FUNC, 7, "this function was marked as unsafe");
declare_warning!(
    SHADOWFUNC_INFINITEARGS,
    8,
    "this function doesn't have strict arguments, use with caution"
);
