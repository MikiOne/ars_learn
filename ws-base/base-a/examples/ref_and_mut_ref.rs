// error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
//  --> ws-base/base-a/examples/ref_and_mut_ref.rs:5:5
//   |
// 3 |     let n = &v[0];
//   |              - immutable borrow occurs here
// 4 |
// 5 |     v.push(0);
//   |     ^^^^^^^^^ mutable borrow occurs here
// 6 |
// 7 |     let x = *n;
//   |             -- immutable borrow later used here
fn main() {
    let mut v = vec![1, 2, 3];
    let n = &v[0];  // 到这里不能再对v进行“可写的操作”

    // 以下这行为什么不会出错呢，那是因为[n]还未被使用，是一个不活跃的。
    // ====所以n一定是要活跃的====
    // 有一个只读引用但是未引用的话，那是不活跃的。
    // 二者直接不会存在互斥的关系
    v.push(0);

    // 这里对n进行解引用，x要读到n地址所指向的内容，
    // 这时候真的对n做了只读的访问，以上又对v做了可写的访问，这两个行为是互斥的
    // let x = *n; // 注释以下行，能编译通过，如果取消注释，则报编译错误
}