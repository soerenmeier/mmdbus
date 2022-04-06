// This code was autogenerated with dbus-codegen-rust
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait ModemSimple {
    fn connect(&self, properties: arg::PropMap) -> Result<dbus::Path<'static>, dbus::Error>;
    fn disconnect(&self, bearer: dbus::Path) -> Result<(), dbus::Error>;
    fn get_status(&self) -> Result<arg::PropMap, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> ModemSimple for blocking::Proxy<'a, C> {

    fn connect(&self, properties: arg::PropMap) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Modem.Simple", "Connect", (properties, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn disconnect(&self, bearer: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Modem.Simple", "Disconnect", (bearer, ))
    }

    fn get_status(&self) -> Result<arg::PropMap, dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Modem.Simple", "GetStatus", ())
            .and_then(|r: (arg::PropMap, )| Ok(r.0, ))
    }
}