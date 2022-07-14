fn main() {
    println!("Hello, world!");

    /*
           定義與實例化 Struct
    */
    // 基礎
    {
        // 如果其中一個 Field 是可變的, 則所有的都是可變的
        struct User {
            username: String,
            email: String,
            sign_in_count: u64,
            active: bool,
        }

        let user = User {
            username: String::from("linonon"),
            email: String::from("qq"),
            sign_in_count: 556,
            active: true,
        };

        println!("user name: {}", user.username);
    }
}
