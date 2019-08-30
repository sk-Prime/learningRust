use colour::*;
use std::io;

struct Account {
    name: String,
    password: String,
    amount: f32,
}

impl Account {
    fn new(name: String, password: String, amount: f32) -> Account {
        let acc = Account {
            name: name,
            password: password,
            amount: amount,
        };
        return acc;
    }
    fn view(&self) {
        print!("\nName      : ");
        green!("{}", self.name);
        print!("\nAmount    : ");
        green!("{}\n", self.amount);
    }
}


struct Bank {
    vault: Vec<Account>,
}

impl Bank {
    fn add_acc(&mut self, acc: Account) {
        self.vault.push(acc);
    }
    fn search(&self, name: String, password: String) -> Result<usize, String> {
        let mut index = 0;
        for acc in &self.vault {
            if name == acc.name && password == acc.password {
                return Ok(index);
            }
            index += 1;
        }
        return Err(format!("there is no account for {}", name));
    }
    fn deposit(&mut self, amount: f32, index: usize) -> Result<String, String> {
        if index < self.vault.len() {
            self.vault[index].amount += amount;
            return Ok(format!(
                "\nDeposit Successful\nYour current balance is {} dollar",
                self.vault[index].amount
            ));
        } else {
            return Err("\nDeposit Unsuccessful".to_string());
        }
    }
    fn withdraw(&mut self, amount: f32, index: usize) -> Result<String, String> {
        if index < self.vault.len() {
            if amount <= self.vault[index].amount {
                self.vault[index].amount -= amount;
                return Ok(format!(
                    "\nHere is your {} dollar\nYour current balance is {} dollar",
                    amount, self.vault[index].amount
                ));
            } else {
                return Err("\nWithdraw Unsuccesful : Insufficient balance".to_string());
            }
        } else {
            return Err("\nWithdraw Unsuccessful".to_string());
        }
    }
    fn view(&self, index: usize) {
        self.vault[index].view();
    }
}

fn main() {
    white_ln!("Trillion Bank");
    let vault: Vec<Account> = Vec::new(); //trillion will take ownership
    let mut trillion = Bank { vault: vault };
    let mut command:String;
    let mut logged_detail = ("".to_string(), "".to_string(), 0, false);
    let mut logged_in:bool;
    let mut logged_name:&str;
    loop {
        logged_in = logged_detail.3;
        logged_name = &logged_detail.0[..];
        print!("-------------------------\n\nWhat do you want? (Active ID :");
        magenta!(logged_name);
        println!(")");
        eprint!("type full word or first letter e.g add or a, about or ab\n\nadd, view, withdraw, deposit, about, login, sign out or exit : ");
        command = input_text("".to_string());
        match &command[..] {
            "exit" | "e" => break,
            "add" | "a" => command_add(&mut trillion),
            "login" | "l" => {
                logged_detail = command_login(&mut trillion);
            }
            "sign out" | "s" => {
                logged_detail = ("".to_string(), "".to_string(), 0, false);
                green_ln!("\nLogged Out");
            }
            "view" | "v" => {
                if logged_in {
                    trillion.view(logged_detail.2);
                } else {
                    red_ln!("\nLogin first or add new account");
                }
            }
            "withdraw" | "w" => {
                if logged_in {
                    command_with(&mut trillion, logged_detail.2);
                } else {
                    red_ln!("\nLogin first or add new account");
                }
            }
            "deposit" | "d" => {
                if logged_in {
                    command_deposit(&mut trillion, logged_detail.2);
                } else {
                    red_ln!("\nLogin first or add new account");
                }
            }
            "about" | "ab" => yellow_ln!("\n\nA simple Bank management program\ncreated using Rust programing language\n\nBy Salim\n\n"),
            _ => red_ln!("\nwrong input"),
        }
    }
}
fn input_text(msg: String) -> String {
    let mut text = String::new();
    eprint!("{}", msg);
    io::stdin().read_line(&mut text).expect("Stdin Error");
    text = text.trim().to_string();
    return text;
}

fn input_float(msg: String) -> f32 {
    let mut text = String::new();
    loop {
        eprint!("{}", msg);
        io::stdin().read_line(&mut text).expect("Stdin Error");
        text = text.trim().to_string();
        match text.parse::<f32>() {
            Ok(n) => {
                return n;
            }
            Err(_) => red_ln!("\nInvalid amount"),
        }
    }
}

fn command_add(trillion: &mut Bank) {
    cyan_ln!("\nAdd account");
    let name = input_text("\ntype your name (c to close): ".to_string());
    if name != "c" {
        let password = input_text("type your password    : ".to_string());
        let acc = Account::new(name.clone(), password.clone(), 0.0);
        trillion.add_acc(acc);
        print!("\n\"{}\" ", name);
        green_ln!("Account Added");
    } else {
        red_ln!("\nAccount Addition Cancelled");
    }
}

fn command_login(trillion: &Bank) -> (String, String, usize, bool) {
    cyan_ln!("\nLogin");

    /*let mut name:String;
    let mut password = String::new();
    let mut index: usize;*/
    loop {
        let name = input_text("\ntype your name (c to close): ".to_string());
        if name == "c" {
            return ("".to_string(), "".to_string(), 0, false);
        }
        let password = input_text("type your password: ".to_string());
        match trillion.search(name.clone(), password.clone()) {
            Ok(n) => {
                let index = n;
                green_ln!("\nLogin Successful");
                return (name, password, index, true);
            }
            Err(e) => {
                let err_msg = &format!("\n{}", e)[..];
                red_ln!(err_msg);
            }
        }
    }
}

fn command_with(trillion: &mut Bank, index: usize) {
    cyan_ln!("\nWithdraw wisely");
    let amount = input_float("\nType the amount (0 to close): ".to_string());
    if amount != 0.0 {
        match trillion.withdraw(amount, index) {
            Ok(s) => println!("{}", s),
            Err(s) => println!("{}", s),
        }
    } else {
        red_ln!("\nWithdraw Cancelled");
    }
}
fn command_deposit(trillion: &mut Bank, index: usize) {
    cyan_ln!("\nDeposit for winter");
    let amount = input_float("\nType the amount (0 to close): ".to_string());
    if amount != 0.0 {
        match trillion.deposit(amount, index) {
            Ok(s) => println!("{}", s),
            Err(s) => println!("{}", s),
        }
    } else {
        red_ln!("\nDeposit Cancelled");
    }
}
