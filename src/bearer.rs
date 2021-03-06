// This code was autogenerated with dbus-codegen-rust
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait Bearer {
    fn connect(&self) -> Result<(), dbus::Error>;
    fn disconnect(&self) -> Result<(), dbus::Error>;
    fn interface(&self) -> Result<String, dbus::Error>;
    fn connected(&self) -> Result<bool, dbus::Error>;
    fn connection_error(&self) -> Result<(String, String), dbus::Error>;
    fn suspended(&self) -> Result<bool, dbus::Error>;
    fn multiplexed(&self) -> Result<bool, dbus::Error>;
    fn ip4_config(&self) -> Result<arg::PropMap, dbus::Error>;
    fn ip6_config(&self) -> Result<arg::PropMap, dbus::Error>;
    fn stats(&self) -> Result<arg::PropMap, dbus::Error>;
    fn ip_timeout(&self) -> Result<u32, dbus::Error>;
    fn bearer_type(&self) -> Result<u32, dbus::Error>;
    fn profile_id(&self) -> Result<i32, dbus::Error>;
    fn properties(&self) -> Result<arg::PropMap, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> Bearer for blocking::Proxy<'a, C> {

    fn connect(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Bearer", "Connect", ())
    }

    fn disconnect(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Bearer", "Disconnect", ())
    }

    fn interface(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Bearer", "Interface")
    }

    fn connected(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Bearer", "Connected")
    }

    fn connection_error(&self) -> Result<(String, String), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Bearer", "ConnectionError")
    }

    fn suspended(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Bearer", "Suspended")
    }

    fn multiplexed(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Bearer", "Multiplexed")
    }

    fn ip4_config(&self) -> Result<arg::PropMap, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Bearer", "Ip4Config")
    }

    fn ip6_config(&self) -> Result<arg::PropMap, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Bearer", "Ip6Config")
    }

    fn stats(&self) -> Result<arg::PropMap, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Bearer", "Stats")
    }

    fn ip_timeout(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Bearer", "IpTimeout")
    }

    fn bearer_type(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Bearer", "BearerType")
    }

    fn profile_id(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Bearer", "ProfileId")
    }

    fn properties(&self) -> Result<arg::PropMap, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Bearer", "Properties")
    }
}
