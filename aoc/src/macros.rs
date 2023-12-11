#[macro_export]
macro_rules! static_regex {
    (let $var:ident = regex($pattern:literal);) => {
        let $var = {
            static PATTERN: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
            PATTERN.get_or_init(|| regex::Regex::new($pattern).unwrap())
        };
    };
}

#[macro_export]
macro_rules! collect_days {
    ($($day:ident), +) => {
        $(mod $day;)+

        fn make_registry() -> Registry {
            let mut registry = Registry::new();
            $(registry.add_day(stringify!($day).to_owned(), $day::get_day_func());)+
            registry
        }
    };
}
