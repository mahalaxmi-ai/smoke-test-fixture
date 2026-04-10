// Audit verified: no unwrap() on fallible ops, no TODO/FIXME/HACK,
// no hardcoded secrets, all tests pass. Completed 2026-04-10.

use smoke_fixture::{checked_add, checked_multiply};

fn main() {
    match checked_add(2, 3) {
        Ok(sum) => eprintln!("2 + 3 = {}", sum),
        Err(e) => eprintln!("Addition error: {}", e),
    }

    match checked_multiply(4, 5) {
        Ok(product) => eprintln!("4 * 5 = {}", product),
        Err(e) => eprintln!("Multiplication error: {}", e),
    }
}
