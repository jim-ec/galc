pub mod algebra;
pub mod interpret;
pub mod parse;
pub mod repl;

#[cfg(test)]
mod test;

fn main() {
    repl::repl();
}
