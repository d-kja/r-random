enum Roles {
    ADMIN,
    MANAGER,
    GUEST,
}

pub fn example() {
    let user_access = Roles::GUEST;
    let can_access_private_files = match user_access {
        Roles::ADMIN => true,
        _ => false,
    };

    println!("can access: {can_access_private_files:?}");
}
