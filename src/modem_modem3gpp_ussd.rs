// This code was autogenerated with dbus-codegen-rust
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait ModemModem3gppUssd {
    fn initiate(&self, command: &str) -> Result<String, dbus::Error>;
    fn respond(&self, response: &str) -> Result<String, dbus::Error>;
    fn cancel(&self) -> Result<(), dbus::Error>;
    fn state(&self) -> Result<u32, dbus::Error>;
    fn network_notification(&self) -> Result<String, dbus::Error>;
    fn network_request(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> ModemModem3gppUssd for blocking::Proxy<'a, C> {

    fn initiate(&self, command: &str) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Modem.Modem3gpp.Ussd", "Initiate", (command, ))
            .and_then(|r: (String, )| Ok(r.0, ))
    }

    fn respond(&self, response: &str) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Modem.Modem3gpp.Ussd", "Respond", (response, ))
            .and_then(|r: (String, )| Ok(r.0, ))
    }

    fn cancel(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Modem.Modem3gpp.Ussd", "Cancel", ())
    }

    fn state(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.Modem3gpp.Ussd", "State")
    }

    fn network_notification(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.Modem3gpp.Ussd", "NetworkNotification")
    }

    fn network_request(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.Modem3gpp.Ussd", "NetworkRequest")
    }
}
