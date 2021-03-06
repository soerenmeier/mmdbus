// This code was autogenerated with dbus-codegen-rust
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait ModemModemCdma {
    fn activate(&self, carrier_code: &str) -> Result<(), dbus::Error>;
    fn activate_manual(&self, properties: arg::PropMap) -> Result<(), dbus::Error>;
    fn activation_state(&self) -> Result<u32, dbus::Error>;
    fn meid(&self) -> Result<String, dbus::Error>;
    fn esn(&self) -> Result<String, dbus::Error>;
    fn sid(&self) -> Result<u32, dbus::Error>;
    fn nid(&self) -> Result<u32, dbus::Error>;
    fn cdma1x_registration_state(&self) -> Result<u32, dbus::Error>;
    fn evdo_registration_state(&self) -> Result<u32, dbus::Error>;
}

#[derive(Debug)]
pub struct ModemModemCdmaActivationStateChanged {
    pub activation_state: u32,
    pub activation_error: u32,
    pub status_changes: arg::PropMap,
}

impl arg::AppendAll for ModemModemCdmaActivationStateChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.activation_state, i);
        arg::RefArg::append(&self.activation_error, i);
        arg::RefArg::append(&self.status_changes, i);
    }
}

impl arg::ReadAll for ModemModemCdmaActivationStateChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(ModemModemCdmaActivationStateChanged {
            activation_state: i.read()?,
            activation_error: i.read()?,
            status_changes: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for ModemModemCdmaActivationStateChanged {
    const NAME: &'static str = "ActivationStateChanged";
    const INTERFACE: &'static str = "org.freedesktop.ModemManager1.Modem.ModemCdma";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> ModemModemCdma for blocking::Proxy<'a, C> {

    fn activate(&self, carrier_code: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Modem.ModemCdma", "Activate", (carrier_code, ))
    }

    fn activate_manual(&self, properties: arg::PropMap) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Modem.ModemCdma", "ActivateManual", (properties, ))
    }

    fn activation_state(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.ModemCdma", "ActivationState")
    }

    fn meid(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.ModemCdma", "Meid")
    }

    fn esn(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.ModemCdma", "Esn")
    }

    fn sid(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.ModemCdma", "Sid")
    }

    fn nid(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.ModemCdma", "Nid")
    }

    fn cdma1x_registration_state(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.ModemCdma", "Cdma1xRegistrationState")
    }

    fn evdo_registration_state(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Modem.ModemCdma", "EvdoRegistrationState")
    }
}
