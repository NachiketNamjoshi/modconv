use std::string::String;

pub trait Convertible {
    fn get_absolute(&self) -> String;
    fn get_symbolic(&self) -> String;
    fn from_absolute(notation: &str) -> Self;
    fn from_symbolic(notation: &str) -> Self;
}

#[derive(Debug)]
pub struct BasicPermission {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

pub struct FilePerm {
    pub user: BasicPermission,
    pub group: BasicPermission,
    pub others: BasicPermission
}

impl Convertible for BasicPermission {
    /**
     * Get Absolute (Octal) Permissions for given FilePerm.
     */
    fn get_absolute(&self) -> String {
        let mut absolute_value = String::from("");
        if self.read {
            absolute_value.push_str("1");
        } else {
            absolute_value.push_str("0");
        }

        if self.write {
            absolute_value.push_str("1");
        } else {
            absolute_value.push_str("0")
        }

        if self.execute {
            absolute_value.push_str("1");
        } else {
            absolute_value.push_str("0")
        }

        let perm: u32 = u32::from_str_radix(&absolute_value.to_owned(), 2).expect("Not a binary number");
        return perm.to_string();
    }

    /**
     * Get Symbolic (Human-Readable) Permissions for given FilePerm
     */
    fn get_symbolic(&self) -> String {
        let mut symbolic_value = String::from("");
        if self.read {
            symbolic_value.push_str("r");
        } else {
            symbolic_value.push_str("-");
        }

        if self.write {
            symbolic_value.push_str("w");
        } else {
            symbolic_value.push_str("-")
        }

        if self.execute {
            symbolic_value.push_str("x");
        } else {
            symbolic_value.push_str("-")
        }
        return symbolic_value;
    }

    fn from_symbolic(notation: &str) -> Self {
        let parts: Vec<char> = notation.chars().collect::<Vec<char>>();

        let mut symbolic_value = BasicPermission {
            read: false,
            write: false,
            execute: false
        };
        symbolic_value.read = parts[0] == 'r';
        symbolic_value.write = parts[1] == 'w';
        symbolic_value.execute = parts[2] == 'x';

        return symbolic_value;
    }


    fn from_absolute(notation: &str) -> Self {
        let mut octal_number: u8 = notation.trim().parse().expect("Given string is not a number.");
        let mut absolute_value = BasicPermission {
            read: false,
            write: false,
            execute: false
        };

        if octal_number == 0 {
            return absolute_value;
        }

        let mut bits: Vec<u8> = Vec::new();
        while octal_number > 0 {
            if octal_number % 2 == 0 {
                bits.push(0);
            } else {
                bits.push(1)
            }

            octal_number = octal_number / 2;
        }

        if bits.len() < 3 {
            let mut diff = 3 - bits.len();
            while diff != 0 {
                bits.push(0);
                diff = diff-1;
            }
        }

        bits.reverse();
        absolute_value.read = bits[0] == 1;
        absolute_value.write = bits[1] == 1;
        absolute_value.execute = bits[2] == 1;

        return absolute_value;
    }

}

impl Convertible for FilePerm {
    /**
     * Get Absolute (Octal) Permissions for given FilePerm.
     */
    fn get_absolute(&self) -> String {
        let mut absolute_value = String::from("");
        absolute_value.push_str(&self.user.get_absolute().to_owned());
        absolute_value.push_str(&self.group.get_absolute().to_owned());
        absolute_value.push_str(&self.others.get_absolute().to_owned());
        return absolute_value;
    }

    /**
     * Get Symbolic (Human-Readable) Permissions for given FilePerm
     */
    fn get_symbolic(&self) -> String {
        let mut symbolic_value = String::from("");
        symbolic_value.push_str(&self.user.get_symbolic().to_owned());
        symbolic_value.push_str(&self.group.get_symbolic().to_owned());
        symbolic_value.push_str(&self.others.get_symbolic().to_owned());
        return symbolic_value;
    }

    fn from_absolute(notation: &str) -> Self {
        let parts: Vec<char> = notation.chars().collect::<Vec<char>>();
        if parts.len() > 3 {
            println!("Not a valid absolute permission notation :: {}", notation);
            std::process::exit(1);
        }

        return FilePerm {
            user: BasicPermission::from_absolute(&String::from(parts[0]).to_owned().to_lowercase()),
            group: BasicPermission::from_absolute(&String::from(parts[1]).to_owned().to_lowercase()),
            others: BasicPermission::from_absolute(&String::from(parts[2]).to_owned().to_lowercase()),
        }
    }

    fn from_symbolic(notation: &str) -> Self {
        let parts: Vec<String> = notation.chars()
                                    .collect::<Vec<char>>()
                                    .chunks(3)
                                    .map(|c| c.iter().collect::<String>())
                                    .collect::<Vec<String>>();
        return FilePerm {
            user: BasicPermission::from_symbolic(&parts[0].to_owned().to_lowercase()),
            group: BasicPermission::from_symbolic(&parts[1].to_owned().to_lowercase()),
            others: BasicPermission::from_symbolic(&parts[2].to_owned().to_lowercase()),
        }
    }
}
