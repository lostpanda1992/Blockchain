// use std::thread;
// // use std::sync::mpsc;
// // use std::time::Duration;
// use std::sync::Mutex;
// use std::rc::Rc;


// fn main() {
//     // let (tx, rx) = mpsc::channel();

//     // let tx1 = mpsc::Sender::clone(&tx);
    
//     // thread::spawn(move || { // move关键字拥有发送端tx的所有权
//     //     let vals = vec![
//     //         String::from("hi"),
//     //         String::from("hello"),
//     //         String::from("world"),
//     //         String::from("thread"),
//     //     ];

//     //     for val in vals {
//     //         tx.send(val).unwrap();
//     //         thread::sleep(Duration::from_millis(1));
//     //     }
//     // });

//     // thread::spawn(move || { // move关键字拥有发送端tx的所有权
//     //     let vals = vec![
//     //         String::from("1hi"),
//     //         String::from("1hello"),
//     //         String::from("1world"),
//     //         String::from("1thread"),
//     //     ];

//     //     for val in vals {
//     //         tx1.send(val).unwrap();
//     //         thread::sleep(Duration::from_millis(1));
//     //     }
//     // });

//     // for received in rx { //将接收端
//     //     println!("got: {}", received)
//     // }

//     // let counter = Rc::new(Mutex::new(0));
//     // let mut handles = vec![];

//     // for _ in 0..10 { //创建十个线程 并将十个线程放到handles中
//     //     let 
//     //     let handle = thread::spawn(move || { //线程闭包中 将counter转移到num中并修改
//     //         let mut num = counter.lock().unwrap();
//     //         *num += 1;
//     //     }); //num离开作用域 释放之后其他线程可以获取锁
//     //     handles.push(handle);
//     // }

//     // for handle in handles { //遍历handles将其中的线程全部释放
//     //     handle.join().unwrap();
//     // }
//     // println!("result: {}",*counter.lock().unwrap()); //尝试获得counter的互斥锁
//     // {
//     //     let mut num = m.lock().unwrap();
//     //     *num = 6;
//     // }
//     // println!("num: {:?}", m);

//     // let b = Box::new(0);
//     // println!("b: {:?}", b);
//     use crate::List::{Cons, Nil};

//     let list = Cons(1,
//         Box::new(Cons(2,
//             Box::new(Cons(3,
//                 Box::new(Nil))))));

// }

// enum List {
//     Cons(i32, Box<List>),
//     Nil
// }

// use crate::List::{Cons, Nil};
// fn main() {
//     let a = Cons(5,
//         Box::new(Cons(10,
//             Box::new(Nil))));
    
//     let b = Cons(3, Box::new(a));
//     let c =  Cons(4, Box::new(a));
// }
// struct MyBox<T>(T);

// impl <T> MyBox<T> {
//     fn new(t: T) -> Self {
//         MyBox(t)
//     }
// }

// use std::ops::Deref;

// impl <T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

fn main() {
    let mut num = 5;
    
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1: {}", *r1);
    }
    // let address = 0x012345usize;
    // let r = address as *const i32;
}