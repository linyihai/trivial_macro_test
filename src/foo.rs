use entry_call::entry_call;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Foo {
    counter: u32,
}

impl Foo {
    pub fn new() -> Self {
        Foo { counter: 0 }
    }

    pub fn get_counter(&self) -> u32 {
        self.counter
    }
    #[entry_call]
    // #[allow(dead_code)]
    // #[allow(unused_variables)]
    pub fn increment(&mut self, _int_v: u32, _bool_v: bool, _foo_v: Foo) {
        self.counter += 1;
    }

    pub fn get_bcs(&self) -> Vec<u8> {
        bcs::to_bytes(self).unwrap()
    }
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
