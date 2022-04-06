// This code was autogenerated with dbus-codegen-rust
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait ModemOma {
    fn setup(&self, features: u32) -> Result<(), dbus::Error>;
    fn start_client_initiated_session(&self, session_type: u32) -> Result<(), dbus::Error>;
    fn accept_network_initiated_session(&self, session_id: u32, accept: bool) -> Result<(), dbus::Error>;
    fn cancel_session(&self) -> Result<(), dbus::Error>;
    fn features(&self) -> Result<u32, dbus::Error>;
    fn pending_network_initiated_sessions(&self) -> Result<Vec<(u32, u32)>, dbus::Error>;
    fn session_type(&self) -> Result<u32, dbus::Error>;
    fn session_state(&self) -> Result<i32, dbus::Error>;
}

#[derive(Debug)]
pub struct ModemOmaSessionStateChanged {
    pub old_session_state: i32,
    pub new_session_state: i32,
    pub session_state_failed_reason: u32,
}

impl arg::AppendAll for ModemOmaSessionStateChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.old_session_state, i);
        arg::RefArg::append(&self.new_session_state, i);
        arg::RefArg::append(&self.session_state_failed_reason, i);
    }
}

impl arg::ReadAll for ModemOmaSessionStateChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(ModemOmaSessionStateChanged {
            old_session_state: i.read()?,
            new_session_state: i.read()?,
            session_state_failed_reason: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for ModemOmaSessionStateChanged {
    const NAME: &'static str = "SessionStateChanged";
    const INTERFACE: &'static str = "org.freedesktop.ModemManager1.Modem.Oma";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> ModemOma for blocking::Proxy<'a, C> {

    fn setup(&self, features: u32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Modem.Oma", "Setup", (features, ))
    }

    fn start_client_initiated_session(&self, session_type: u32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Modem.Oma", "StartClientInitiatedSession", (session_type, ))
    }

    fn accept_network_initiated_session(&self, session_id: u32, accept: bool) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Modem.Oma", "AcceptNetworkInitiatedSession", (session_id, accept, ))
    }

    fn cancel_session(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Modem.Oma", "CancelSession", ())
    }

    fn features(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.Oma", "Features")
    }

    fn pending_network_initiated_sessions(&self) -> Result<Vec<(u32, u32)>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.Oma", "PendingNetworkInitiatedSessions")
    }

    fn session_type(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.Oma", "SessionType")
    }

    fn session_state(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.Oma", "SessionState")
    }
}
