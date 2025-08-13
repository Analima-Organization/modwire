/// Flattens the module structure for ergonomic consumption.
/// # Example
/// ```rs
/// expose!(
///     pub foo
///     pub bar
///         foo_bar
/// );
/// ```
/// # Example Expansion
/// ```rs
/// mod foo; pub use foo::*;
/// mod bar; pub use bar::*;
/// mod foo_bar; use foo_bar::*;
/// ```
#[macro_export]
macro_rules! expose {
    ($($vis:vis $module:ident)*) => {
        $(
            mod $module; $vis use $module::*;
        )*
    };
}