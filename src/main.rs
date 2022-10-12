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

    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            sign_in_count: 1,
            active: true,
        }
    }

    let results =  build_user(user1.email, user1.username);
    println!("I know this is not the best example but the function results are: {:?}, {:?}, {:?}, {:?}", results.username, results.email, results.active, results.sign_in_count);
   
}


