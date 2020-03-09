pub fn timeArg<T, R, F: Fn(T) -> R>(f: F, arg: T) -> (R, Duration) {
    let start = Instant::now();
    let res = f(arg);
    let duration = start.elapsed();
    (res, duration)
}

pub fn time<R, F: Fn() -> R>(f: F) -> (R, Duration) {
    let start = Instant::now();
    let res = f();
    let duration = start.elapsed();
    (res, duration)
}
