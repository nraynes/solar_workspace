#[macro_export]
macro_rules! prod_switch {
    ( $dev:expr, $release:expr ) => {{
        #[cfg(debug_assertions)]
        {
            $dev
        }

        #[cfg(not(debug_assertions))]
        {
            $release
        }
    }};
}
