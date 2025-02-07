use core::mem::MaybeUninit;

pub macro const_map($f:expr, $a:expr) {
    match ($f, $a) {
        (f, a) => {
            const fn map_to_uninit<T, U, const N: usize>(_: &[T; N]) -> [MaybeUninit<U>; N] {
                [const { MaybeUninit::uninit() }; N]
            }

            let a = MaybeUninit::new(a).transpose();
            let mut b = map_to_uninit(&a);
            let mut i = 0;
            while i < b.len() {
                // SAFETY: `a` was created by `MaybeUninit::new`; every element of `a` is read
                // exactly once.
                b[i].write(f(unsafe { a[i].assume_init_read() }));
                i += 1;
            }
            // SAFETY: all elements of `b` are initialized.
            unsafe { MaybeUninit::array_assume_init(b) }
        }
    }
}
