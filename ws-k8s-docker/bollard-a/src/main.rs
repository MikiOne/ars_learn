fn main() {
    let mut u = 0i32;
    let mut v = 1i32;
    let mut w = 2i32;

    // lifetime of `a` = α ∪ β ∪ γ
    let mut a = &mut u;     // --+ α. lifetime of `&mut u`  --+ lexical "lifetime" of `&mut u`,`&mut u`, `&mut w` and `a`
    use(a);                 //   |                            |
    *a = 3; // <-----------------+                            |
    ...                     //                                |
        a = &mut v;             // --+ β. lifetime of `&mut v`    |
    use(a);                 //   |                            |
    *a = 4; // <-----------------+                            |
    ...                     //                                |
        a = &mut w;             // --+ γ. lifetime of `&mut w`    |
    use(a);                 //   |                            |
    *a = 5; // <-----------------+ <--------------------------+
}

