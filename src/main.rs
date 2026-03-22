use std::marker::PhantomData;
use std::sync::{Mutex, MutexGuard};

// 1. The Hierarchy (Labels)
struct Ip;
struct Device;

// 2. The Rules (Graph)
trait CanLock<Next> {}
struct Unlocked; // Initial state
impl CanLock<Ip> for Unlocked {} // Ok: IP first
impl CanLock<Device> for Ip {} // Ok: Device is allowed after IP
// Note: There is no impl for Device -> Ip, so that is forbidden!

// 3. The Wrapper
struct OrderedMutex<Id, T> {
    inner: Mutex<T>,
    _id: PhantomData<Id>,
}

impl<Id, T> OrderedMutex<Id, T> {
    pub fn new(data: T) -> Self {
        Self {
            inner: Mutex::new(data),
            _id: PhantomData,
        }
    }

    // The 'magic': locking requires a token of the correct 'previous' state
    pub fn lock<'a, Current>(&'a self, _proof: &Locked<Current>) -> (MutexGuard<'a, T>, Locked<Id>)
    where
        Current: CanLock<Id>,
    {
        (self.inner.lock().unwrap(), Locked(PhantomData))
    }
}

// The proof that a thread holds
struct Locked<State>(PhantomData<State>);

fn main() {
    let ip_mutex = OrderedMutex::<Ip, String>::new("192.168.1.1".into());
    let dev_mutex = OrderedMutex::<Device, String>::new("Ethernet0".into());

    // We start without locks
    let root_token = Locked(PhantomData::<Unlocked>);

    // --- SCENARIO A: The correct way ---
    let (ip_data, ip_token) = ip_mutex.lock(&root_token);
    let (dev_data, _dev_token) = dev_mutex.lock(&ip_token);
    println!("Working with {} on {}", *ip_data, *dev_data);

    // --- SCENARIO B: The forbidden way ---
    let _root_token_2 = Locked(PhantomData::<Unlocked>);

    // Step 1: Locking Device is allowed (since we have no rules against Device first yet)
    // But wait, to make this truly watertight we would also
    // need to add `impl CanLock<Device> for Unlocked`.
    // Let's assume we only allow IP as a starting point:

    // let (dev_data, dev_token) = dev_mutex.lock(&_root_token_2); // Error! Unlocked cannot lock Device.
}
