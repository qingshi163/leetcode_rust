#[macro_export]
macro_rules! vec_string {
    ($($e:expr),*) => {vec![$($e.to_string()), *]};
    ($($e:expr,)*) => {vec![$($e.to_string()), *]};
}
#[macro_export]
macro_rules! vec_sort {
    ($($e:expr),*) => {{
        let mut tmp = vec![$($e),*];
        tmp.sort();
        tmp
    }};
    ($($e:expr,)*) => {{
        let mut tmp = vec![$($e),*];
        tmp.sort();
        tmp
    }};
}