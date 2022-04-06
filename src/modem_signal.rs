// This code was autogenerated with dbus-codegen-rust
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait ModemSignal {
    fn setup(&self, rate: u32) -> Result<(), dbus::Error>;
    fn rate(&self) -> Result<u32, dbus::Error>;
    fn cdma(&self) -> Result<arg::PropMap, dbus::Error>;
    fn evdo(&self) -> Result<arg::PropMap, dbus::Error>;
    fn gsm(&self) -> Result<arg::PropMap, dbus::Error>;
    fn umts(&self) -> Result<arg::PropMap, dbus::Error>;
    fn lte(&self) -> Result<arg::PropMap, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> ModemSignal for blocking::Proxy<'a, C> {

    fn setup(&self, rate: u32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Modem.Signal", "Setup", (rate, ))
    }

    fn rate(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.Signal", "Rate")
    }

    fn cdma(&self) -> Result<arg::PropMap, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.Signal", "Cdma")
    }

    fn evdo(&self) -> Result<arg::PropMap, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.Signal", "Evdo")
    }

    fn gsm(&self) -> Result<arg::PropMap, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.Signal", "Gsm")
    }

    fn umts(&self) -> Result<arg::PropMap, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.Signal", "Umts")
    }

    fn lte(&self) -> Result<arg::PropMap, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.Signal", "Lte")
    }
}