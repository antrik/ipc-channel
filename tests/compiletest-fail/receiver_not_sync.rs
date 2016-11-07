extern crate ipc_channel;

struct SyncThing<T: Sync>(T);

struct Invalid(SyncThing<ipc_channel::platform::OsIpcSender>);
            //~^ error: the trait bound `std::cell::Cell<()>: std::marker::Sync` is not satisfied
