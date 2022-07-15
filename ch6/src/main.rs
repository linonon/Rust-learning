fn main() {
    println!("Hello, world!");

    /*
           6 枚舉
    */

    // 枚舉值
    {
        enum IPAddKind {
            V4,
            V6,
        }

        let IPv4 = IPAddKind::V4;
        let IPv6 = IPAddKind::V6;

        route(IPv4);
        route(IPv6);

        fn route(ip: IPAddKind) {}
    }
    {
        enum IPAddKind {
            V4,
            V6,
        }

        struct IPAddr {
            kind: IPAddKind,
            address: String,
        }

        let home = IPAddr {
            kind: IPAddKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IPAddr {
            kind: IPAddKind::V6,
            address: String::from("::1"),
        };

        enum IPAddrKindV2 {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let homeV2 = IPAddrKindV2::V4(127, 0, 0, 1);
        let lookbackV2 = IPAddrKindV2::V6(String::from("::1"));
    }
    // Message
    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        impl Message {
            fn call(&self) {}
        }
    }
    // Option
    {
        // Rust 沒有 Null
        // Rust: Option<T>
        // enum Option<T> {
        //     Some(T),
        //     None,
        // }

        let num = Some(5);
        let string = Some("A String");

        let absent_number: Option<i32> = None;
    }
}
