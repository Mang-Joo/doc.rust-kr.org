fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // ANCHOR: here
    impl Message {
        fn call(&self) {
            // 메서드 본문이 여기 정의될 것입니다
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
    // ANCHOR_END: here
}
