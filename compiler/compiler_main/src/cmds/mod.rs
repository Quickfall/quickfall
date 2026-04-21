pub mod build;
pub mod check;

#[macro_export]
macro_rules! quietlyquit_if_errors {
    () => {
        if (diagnostics::has_diagnostics()) {
            diagnostics::dump_diagnostics();
            std::process::exit(445);
        }
    };
}

#[macro_export]
macro_rules! soft_panic {
    ($lit:literal) => {
        println!($lit);
        std::process::exit(445)
    };
}
