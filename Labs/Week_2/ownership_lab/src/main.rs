struct User{
    name: String,
    age: u8,
}

fn consumer_user(user: User){
    println!("Consuming user: {}", user.name);
}

fn read_user(user: &User){
    println!("Reading user: {}", user.name);
}

fn birthday(user: &mut User){
    user.age += 1;
    println!("Happy Birthday, {}!", user.name);
}

fn main() {
    let user1 = User{
        name: String::from("Alice"),
        age: 30,
    };

    let user2 = User{
        name: String::from("Bob"),
        age: 28,
    };

    let mut user3 = User{
        name: String::from("Carol"),
        age: 25,
    };

    let mut user4 = User{
        name: String::from("Dave"),
        age: 40,
    };

    consumer_user(user1);
    read_user(&user2);
    birthday(&mut user3);

    let r1 = &user4;
    println!("{}", r1.name);
    let r2 = &mut user4; 
    r2.age += 1;

    // println!("{} is {} years old", user1.name, user1.age);
    println!("Still accessible: {}", user2.name);
    println!("{} is now {} years old", user3.name, user3.age);
    // Compile-time error: cannot borrow `user4` as mutable because it is also borrowed as immutable
    // println!("{} {}", r1.name, r2.age); 
}
