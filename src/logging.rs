use std::fmt::Display;

use colored::*;

pub fn log_addition(message: impl Display) {
    println!("{} {}", "[+]".green(), message);
}
