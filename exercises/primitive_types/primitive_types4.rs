// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    let a = [0; 1000];
    // 假设你需要获取前10个元素的切片
    let nice_slice = &a[0..10];

    // 使用调试格式化器打印切片
    println!("nice_slice={:?}", nice_slice);

    // 或者使用多行美观的调试格式化器
    println!("nice_slice={:#?}", nice_slice);
}
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];


    assert_eq!([2, 3, 4], nice_slice)
}
