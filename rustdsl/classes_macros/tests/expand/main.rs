macro_rules! expand_test_case {
    (class $test:ident {$($input:tt)*} => {$($output:tt)*}) => {
        #[cfg(not(miri))]
        #[test]
        fn $test() {
            ::pretty_assertions::assert_eq!(
                format!("{}", ::classes_macros::class_quoted! { $($input)* })
                    .replace(";", ";\n")
                    .replace(",", ",\n")
                    .replace("{", "{\n"),
                format!("{}", ::quote::quote! { $($output)* })
                    .replace(";", ";\n")
                    .replace(",", ",\n")
                    .replace("{", "{\n"),
            );
            // ::classes_macros::class! { $($input)* }
        }
    };
    ($test:ident {$($input:tt)*} => {$($output:tt)*}) => {
        #[cfg(not(miri))]
        #[test]
        fn $test() {
            ::pretty_assertions::assert_eq!(
                format!("{}", ::classes_macros::classes_quoted! { $($input)* })
                    .replace(";", ";\n")
                    .replace(",", ",\n")
                    .replace("{", "{\n"),
                format!("{}", ::quote::quote! { $($output)* })
                    .replace(";", ";\n")
                    .replace(",", ",\n")
                    .replace("{", "{\n"),
            );
            // ::classes_macros::classes! { $($input)* }
        }
    };
}

mod gallery_page;
mod mixin;
mod no_super;
