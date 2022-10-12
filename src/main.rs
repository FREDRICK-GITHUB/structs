fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user1 = User {
        username: String::from("fredrick"),
        email: String::from("fredmainahk@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    user1.sign_in_count = 3;
    user1.email = String::from("kamaufredm@gmail.com");

    println!("my username is: {}", user1.username);
    println!("my email address is: {}", user1.email);
    println!("my sign in count is: {}", user1.sign_in_count);
    println!("my status is: {}", user1.active);
}
