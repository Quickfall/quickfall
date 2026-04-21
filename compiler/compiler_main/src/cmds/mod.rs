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
