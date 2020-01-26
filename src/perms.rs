pub fn perms_to_str(mut perm_oct: u32) -> String {
    let mut perm_str = String::from("");
    let others = perm_oct%8;
    perm_oct/=8;
    let group = perm_oct%8;
    perm_oct/=8;
    let owner = perm_oct%8;

    perm_str.push_str(&octal_to_str(owner));
    perm_str.push_str(&octal_to_str(group));
    perm_str.push_str(&octal_to_str(others));

    perm_str
}

pub fn octal_to_str(oct: u32) -> String {
    let mut perm = String::from("");
    if (oct >> 2) & 1 == 1 {
        perm.push('r');
    } else {
        perm.push('-');
    }

    if (oct >> 1) & 1 == 1 {
        perm.push('w');
    } else {
        perm.push('-');
    }

    if oct & 1 == 1 {
        perm.push('x');
    } else {
        perm.push('-');
    }

    perm
}
