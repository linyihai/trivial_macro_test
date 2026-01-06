#![feature(int_from_ascii)]

mod foo;

use foo::Foo;

fn main() {
    let mut foo = Foo::new();
    println!("Initial counter: {}", foo.get_counter());
    foo.increment(1, true, Foo::new());
    println!("Counter after increment: {}", foo.get_counter());
    let mut args = foo::Args::new(vec![
        vec![49],      // ASCII for '1' -> u32
        vec![49],      // ASCII for '1' -> bool (true)
        foo.get_bcs(), // BCS encoded Foo
    ]);
    foo.entry_increment(&mut args);
    println!("counter after entry_call: {}", foo.get_counter());
}
