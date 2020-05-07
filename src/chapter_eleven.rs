// This file test the User struct in chapter_five.rs

use crate::chapter_five::*;

#[test]
fn test_get_sign_in_count() {
    let mut u1 = User::new(String::from("u1"), String::from("u1@test.com"));
    let mut u2 = User::new(String::from("u2"), String::from("u2@test.com"));
    u1.sign_in();

    assert_eq!(u1.get_sign_in_count(), 2);
    assert_eq!(u2.get_sign_in_count(), 1);
}

#[test]
#[should_panic]  // passes if panics
fn test_new_invalid_email() {
    let u3 = User::new(String::from("u3"), String::from("u3_email"));
    println!("u3: {}", u3.get_sign_in_count());
}
