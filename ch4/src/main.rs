fn main() {
    println!("Hello, world!");
    // Stack: LIFO, 入站出站
    // Heap: 通過 指針訪問數據, 比 Stack 慢

    // 所有權, Rust 在編譯期間就會做好這一步, 則不需要運行時(rumtime)
    // - 那跟蹤代碼的哪些部分正在使用 heap 的那些數據
    // - 最小化 heap 上的重複數據量
    // - 清理 heap 上未使用的數據以避免空間不足

    // 所有權規則
    // 每個值都有一個變量, 這個變量是該值的所有者
    // 每個值同時只能有一個所有者
    // 當所有者超出作用域(scpoe)時, 該值將被刪除

    // String: heap 上分配
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s)

    // ↑ s 走出作用域後, 內存會自動釋放, drop()
}
