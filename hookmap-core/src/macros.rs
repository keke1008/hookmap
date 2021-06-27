#[macro_export]
macro_rules! bihashmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(bihashmap!(@single $rest)),*]));

    ($($left:expr => $right:expr,)+) => { bihashmap!($($left => $right),+) };
    ($($left:expr => $right:expr),*) => {
        {
            let _cap = bihashmap!(@count $($left),*);
            let mut _bihashmap = bimap::BiHashMap::with_capacity(_cap);
            $(
                _bihashmap.insert($left, $right);
            )*
            _bihashmap
        }
    };
}
