// This code was autogenerated with dbus-codegen-rust
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait Sms {
    fn send(&self) -> Result<(), dbus::Error>;
    fn store(&self, storage: u32) -> Result<(), dbus::Error>;
    fn state(&self) -> Result<u32, dbus::Error>;
    fn pdu_type(&self) -> Result<u32, dbus::Error>;
    fn number(&self) -> Result<String, dbus::Error>;
    fn text(&self) -> Result<String, dbus::Error>;
    fn data(&self) -> Result<Vec<u8>, dbus::Error>;
    fn smsc(&self) -> Result<String, dbus::Error>;
    fn validity(&self) -> Result<(u32, arg::Variant<Box<dyn arg::RefArg + 'static>>), dbus::Error>;
    fn class(&self) -> Result<i32, dbus::Error>;
    fn teleservice_id(&self) -> Result<u32, dbus::Error>;
    fn service_category(&self) -> Result<u32, dbus::Error>;
    fn delivery_report_request(&self) -> Result<bool, dbus::Error>;
    fn message_reference(&self) -> Result<u32, dbus::Error>;
    fn timestamp(&self) -> Result<String, dbus::Error>;
    fn discharge_timestamp(&self) -> Result<String, dbus::Error>;
    fn delivery_state(&self) -> Result<u32, dbus::Error>;
    fn storage(&self) -> Result<u32, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> Sms for blocking::Proxy<'a, C> {

    fn send(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Sms", "Send", ())
    }

    fn store(&self, storage: u32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.ModemManager1.Sms", "Store", (storage, ))
    }

    fn state(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Sms", "State")
    }

    fn pdu_type(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Sms", "PduType")
    }

    fn number(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Sms", "Number")
    }

    fn text(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Sms", "Text")
    }

    fn data(&self) -> Result<Vec<u8>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Sms", "Data")
    }

    fn smsc(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Sms", "SMSC")
    }

    fn validity(&self) -> Result<(u32, arg::Variant<Box<dyn arg::RefArg + 'static>>), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Sms", "Validity")
    }

    fn class(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Sms", "Class")
    }

    fn teleservice_id(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Sms", "TeleserviceId")
    }

    fn service_category(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Sms", "ServiceCategory")
    }

    fn delivery_report_request(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Sms", "DeliveryReportRequest")
    }

    fn message_reference(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Sms", "MessageReference")
    }

    fn timestamp(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Sms", "Timestamp")
    }

    fn discharge_timestamp(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Sms", "DischargeTimestamp")
    }

    fn delivery_state(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Sms", "DeliveryState")
    }

    fn storage(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.ModemManager1.Sms", "Storage")
    }
}
