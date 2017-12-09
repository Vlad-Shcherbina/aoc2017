#![macro_use]

#[macro_export]
macro_rules! debug {
    ($($x:expr),+) => {
        $(
        print!("{} = {:?}  ", stringify!($x), $x);
        )*
        println!();
    };
}
