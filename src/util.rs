#[macro_export]
macro_rules! compose {
    ($f: expr, $g: expr) => {
        move |t| {
            let u = $f(t);
            $g(u)
        }
    };
}

#[macro_export]
macro_rules! curry {
    ($f: expr) => {
        move |t| move |u| $f(t, u)
    };
}
