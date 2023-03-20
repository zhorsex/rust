// 冒泡排序算法
fn bubble_sort(mut v: Vec<i32>) -> Vec<i32> {
    let len = v.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if v[j] > v[j + 1] {
                // 交换两个元素
                v.swap(j, j + 1);
            }
        }
    }
    v
}

fn main() {
    // 测试数据
    let v = vec![2, 2, 10, 1, 5, 4, 3];
    // 调用冒泡排序函数
    let sorted_v = bubble_sort(v);
    // 打印结果
    println!("{:?}", sorted_v); // [1, 2, 2, 3, 4, 5, 10]
}
