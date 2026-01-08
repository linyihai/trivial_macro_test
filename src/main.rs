mod foo;

use foo::Foo;

fn main() {
    let mut foo = Foo::new();
    let mut ctx = foo::TxContext::new();
    println!("Initial counter: {}", foo.get_counter());
    foo.increment(1, true, Foo::new(), &mut ctx);
    println!("Counter after increment: {}", foo.get_counter());
    let mut args = foo::Args::new(vec![
        foo.get_bcs(),
        vec![49],      // ASCII for '1' -> u32
        vec![49],      // ASCII for '1' -> bool (true)
        foo.get_bcs(), // BCS encoded Foo
    ]);
    Foo::entry_increment(&mut args, &mut ctx);
    println!("counter after entry_call: {}", foo.get_counter());
    let mut args = foo::Args::new(vec![foo.get_bcs()]);
    println!(
        "counter after entry_call: {:?}",
        Foo::entry_get_counter(&mut args, &mut ctx)
    );

    let a = foo::entry_foo_function(
        &mut foo::Args::new(vec![
            vec![52, 50, 55], // ASCII for '42' -> u32
            vec![49],         // ASCII for '1' -> bool (true)
            foo.get_bcs(),    // BCS encoded Foo
        ]),
        &mut ctx,
    );

    print!("entry_foo_function returned: {}", a);
}
