use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use std::ops::Deref;

static REGISTRY: AtomicUsize = ATOMIC_USIZE_INIT;
const UNINITIALIZED: usize = 0;
const INITIALIZING: usize = 1;

pub struct Registry {
    vars: Mutex<HashMap<String, &'static Var>>
}

impl Registry {
    pub fn publish<N>(&self, name: N, var: &'static Var) where N: Into<String> {
        let mut vars = self.vars.lock().unwrap();
        vars.insert(name.into(), var);
    }
}

pub trait Var {
    fn value(&self) -> String;
}

pub struct Func {
    inner: Fn() -> String
}

impl Var for Func {
    fn value(&self) -> String {
        (self.inner)()
    }
}

pub struct Int {
    inner: AtomicUsize
}

impl Var for Int {
    fn value(&self) -> String {
        self.inner.load(Ordering::SeqCst).to_string()
    }
}


struct RegistryGuard(usize);

impl Deref for RegistryGuard {
    type Target = Registry;

    fn deref(&self) -> &'static Registry {
        unsafe { std::mem::transmute(self.0) }
    }
}

fn registry() -> Option<RegistryGuard> {
    let reg = REGISTRY.load(Ordering::SeqCst);
    if reg == UNINITIALIZED || reg == INITIALIZING {
        None
    } else {
        Some(RegistryGuard(reg))
    }
}

pub fn publish<N>(name: N, var: &'static Var) where N: Into<String> {
    if let Some(reg) = registry() {
        reg.deref().publish(name, var);
    }
}


pub fn vars() -> Option<Vec<(String, &'static Var)>> {
    registry().map(|r| {
        let vars = r.vars.lock().unwrap();
        vars.clone().into_iter().collect::<Vec<(String, &'static Var)>>()
    })
}

// http://doc.rust-lang.org/std/sync/struct.Once.html
pub fn init() {
    let registry = Box::new(
        Registry {
            vars: Mutex::new(HashMap::new())
        }
    );
    let registry = unsafe {
        std::mem::transmute::<Box<Registry>, usize>(registry)
    };
    REGISTRY.store(registry, Ordering::SeqCst);
}

//fn publish(name: &str, )

#[test]
fn it_works() {
}
