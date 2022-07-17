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
        let normal_number: i32 = 1;

        // 无法转换
        // let tmp = absent_number + normal_number;
    }

    // match
    {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }

        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => {
                    println!("penny");
                    1
                }
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter form {:?}!", state);
                    25
                }
            }
        }

        let c = Coin::Quarter(UsState::Alaska);
        println!("{:?}", value_in_cents(c))
    }

    // match: 匹配 Option<T>
    {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

        // fn plus_one(x: Option<i32>) -> Option<i32> {
        // match 匹配需要全部处理, 不可以少
        //     match x {
        //         None => None,
        //         Some(i) => Some(i + 1),
        //     }
        // }
        fn plus_one(x: Option<i32>) -> Option<i32> {
            x.map(|i| i + 1)
        }

        println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
    }
    // match 通配符
    // _ : 通配符
    {
        let v = 0u8;
        match v {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => println!("{}", v),
        }
    }

    // if let
    // 处理只关心一种匹配而忽略其他匹配的情况
    // less typing, less indentation, less boilerplate code.
    {
        let v = Some(0u8);

        if let Some(3) = v {
            println!("three")
        } else {
            println!("v: {:?}", v)
        }

        // why we use `if let` not `if ==`
        // if Some(3) == v {
        //     println!("three")
        // } else {
        //     println!("v: {:?}", v)
        // }
        enum Choices {
            A,
            B,
            C(i32),
        }

        let choices: Choices = Choices::C(2);
        if let Choices::C(value) = choices {
            println!("{}", value * 2)
        }
    }
}
