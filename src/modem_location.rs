// This code was autogenerated with dbus-codegen-rust
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait ModemLocation {
    fn setup(&self, sources: u32, signal_location: bool) -> Result<(), dbus::Error>;
    fn get_location(&self) -> Result<::std::collections::HashMap<u32, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error>;
    fn set_supl_server(&self, supl: &str) -> Result<(), dbus::Error>;
    fn set_gps_refresh_rate(&self, rate: u32) -> Result<(), dbus::Error>;
    fn capabilities(&self) -> Result<u32, dbus::Error>;
    fn enabled(&self) -> Result<u32, dbus::Error>;
    fn signals_location(&self) -> Result<bool, dbus::Error>;
    fn location(&self) -> Result<::std::collections::HashMap<u32, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error>;
    fn supl_server(&self) -> Result<String, dbus::Error>;
    fn gps_refresh_rate(&self) -> Result<u32, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> ModemLocation for blocking::Proxy<'a, C> {

    fn setup(&self, sources: u32, signal_location: bool) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Modem.Location", "Setup", (sources, signal_location, ))
    }

    fn get_location(&self) -> Result<::std::collections::HashMap<u32, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Modem.Location", "GetLocation", ())
            .and_then(|r: (::std::collections::HashMap<u32, arg::Variant<Box<dyn arg::RefArg + 'static>>>, )| Ok(r.0, ))
    }

    fn set_supl_server(&self, supl: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Modem.Location", "SetSuplServer", (supl, ))
    }

    fn set_gps_refresh_rate(&self, rate: u32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Modem.Location", "SetGpsRefreshRate", (rate, ))
    }

    fn capabilities(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.Location", "Capabilities")
    }

    fn enabled(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.Location", "Enabled")
    }

    fn signals_location(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.Location", "SignalsLocation")
    }

    fn location(&self) -> Result<::std::collections::HashMap<u32, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.Location", "Location")
    }

    fn supl_server(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.Location", "SuplServer")
    }

    fn gps_refresh_rate(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.Location", "GpsRefreshRate")
    }
}
