use entry_call::{entry_call_function, entry_call_method};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Foo {
    counter: u32,
}

impl Foo {
    pub fn new() -> Self {
        Foo { counter: 0 }
    }

    #[entry_call_method]
    pub fn get_counter(&self) -> u32 {
        self.counter
    }
    #[entry_call_method]
    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn increment(&mut self, _int_v: u32, _bool_v: bool, _foo_v: Foo, _ctx: &mut TxContext) {
        self.counter += 1;
    }

    pub fn get_bcs(&self) -> Vec<u8> {
        bcs::to_bytes(self).unwrap()
    }

    #[entry_call_method]
    pub fn new_foo(self) -> Foo {
        Foo {
            counter: self.counter,
        }
    }

    #[entry_call_method]
    pub fn generic_method_no_arg<T: for<'a> serde::Deserialize<'a>>(self) {}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Bar<T> {
    value: T,
}

#[entry_call_function]
pub fn mut_foo(_a: &mut Foo) {}

#[entry_call_function]
pub fn ref_foo(_a: &Foo) {}

#[entry_call_function]
pub fn generic_struct<T: for<'a> serde::Deserialize<'a>>(_b: Bar<T>) {}

#[entry_call_function]
pub fn generic_method<T: for<'a> serde::Deserialize<'a>>(_val: T) {
    // do nothing
}

#[entry_call_function]
pub fn generic_method_no_arg<T: for<'a> serde::Deserialize<'a>>() {}

#[entry_call_function]
fn foo_function(int_a: u32, _bool: bool, _foo: Foo, _ctx: &mut TxContext) -> u32 {
    int_a
}

pub struct Args {
    inner: Vec<Vec<u8>>,
}

impl Iterator for Args {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            None
        } else {
            Some(self.inner.remove(0))
        }
    }
}

impl Args {
    pub fn new(args: Vec<Vec<u8>>) -> Self {
        Args { inner: args }
    }
    pub fn next_pure(&mut self) -> Option<Vec<u8>> {
        if self.inner.is_empty() {
            None
        } else {
            Some(self.inner.remove(0))
        }
    }
}

pub struct TxContext;

impl TxContext {
    pub fn new() -> Self {
        TxContext {}
    }
}

fn fast_ascii_to_u64(bytes: &[u8]) -> u64 {
    let mut value: u64 = 0;
    let mut i = 0;

    while i < bytes.len() {
        if !bytes[i].is_ascii_digit() {
            panic!("Invalid digit found");
        }
        let digit = (bytes[i] - b'0') as u64;

        if value > (u64::MAX - digit) / 10 {
            panic!("u64 overflow");
        }

        value = value * 10 + digit;
        i += 1;
    }

    value
}
