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

    //this struct shows how we can use some values that were declared in another struct instance by directly referencing that struct
    let mut user2 = User {
        email: String::from("fredmainahk"),
        username: String::from("freddie"),
        sign_in_count: user1.sign_in_count,
        active: user1.active,
    };

    //this implementation show how we can update only some values and reuse others from another struct
    let mut user3 = User {
        username: String::from("maina"),
        email: String::from("kamaufredm@gmail.com"),
        ..user1
    };

    println!("We have two users with 3 user names which are: {} and: {} aand: {}", user1.username, user2.username, user3.username);

    
   
}


