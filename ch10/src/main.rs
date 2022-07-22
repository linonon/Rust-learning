use std::fmt::{Debug, Display};

fn main() {
    println!("Hello, world!");
    // 消除重複代碼
    {
        fn max<T: PartialOrd + Clone>(list: &[T]) -> &T {
            // fn max(list: &[i32]) -> i32 {
            // let mut max_num = list[0].clone();
            // for item in list.iter() {
            //     if *item > max_num {
            //         max_num = item.clone();
            //     }
            // }
            let mut max_num = &list[0];
            for item in list.iter() {
                if item > max_num {
                    max_num = item;
                }
            }
            max_num
        }
        let num_list = vec![1, 2, 5, 4, 3];
        let result = max(&num_list);
        println!("{}", result);

        let char_list = vec!['a', 'b', 'c', 'd', 'e'];
        let result = max(&char_list);
        println!("{}", result);

        let str_list = vec![String::from("hello"), String::from("world")];
        let result = max(&str_list);
        println!("{}", result);
    }
    // Struct 泛型
    {
        // 類型過多時, 應該 重組單元
        struct Point<T, U> {
            x: T,
            y: U,
        }

        let integer = Point { x: 1, y: 2 };
        let float = Point { x: 1.1, y: 2.2 };
    }
    // Enum 泛型
    {
        struct Point<T> {
            x: T,
            y: T,
        }

        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
            fn y(&self) -> &T {
                &self.y
            }
        }

        let point = Point { x: 1, y: 2 };
        println!("{}, {}", point.x(), point.y());
    }
    // Trait 特製, 特征
    {
        pub trait Summary {
            fn summerize(&self) -> String;
        }

        pub struct News {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub contents: String,
        }

        impl Summary for News {
            fn summerize(&self) -> String {
                format!("{} {}", self.headline, self.location)
            }
        }

        let news = News {
            headline: String::from("GG"),
            location: String::from("Home"),
            author: String::from("linonon"),
            contents: String::from("a;lsdfjka;ldsfk"),
        };

        println!("{}", news.summerize());

        pub struct Tweet {
            pub username: String,
            pub contents: String,
            pub reply: String,
            pub retweets: String,
        }

        impl Summary for Tweet {
            fn summerize(&self) -> String {
                format!("{} {}", self.username, self.contents)
            }
        }

        // Trait 作為參數
        pub fn notify_impl_trait(item: impl Summary) {
            println!("Breaking news: {}", item.summerize());
        }

        pub fn notify_trait_bound<T: Summary>(item: T) {
            println!("Breaking news: {}", item.summerize());
        }

        pub fn notify_trait_plus<T: Summary + Display>(item: T) {}

        pub fn notify_where<T, U>(item: T, item2: U)
        where
            T: Display + Summary,
            U: Clone + Debug,
        {
            todo!()
        }
    }
    /*
           生命週期
    */
    {
        // let r;
        // {
        //     let b = 1;
        //     r = &b;
        // }
        // println!("r: {}", r)

        let string = String::from("abdc");
        let string2 = "xyz";

        let string_max = max(string.as_str(), string2);
        println!("string max: {}", string_max);

        fn max<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                return x;
            }
            y
        }
    }
}
