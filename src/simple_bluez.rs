mod interface {
    use zbus_macros::proxy;
    #[proxy(
        interface = "org.bluez.Adapter1",
        default_service = "org.bluez",
        default_path = "/org/bluez/hci0"
    )]
    pub trait Adapter {
        /// GetDiscoveryFilters method
        fn get_discovery_filters(&self) -> zbus::Result<Vec<String>>;
    
        /// RemoveDevice method
        fn remove_device(&self, device: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;
    
        /// SetDiscoveryFilter method
        fn set_discovery_filter(
            &self,
            properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        ) -> zbus::Result<()>;
    
        /// StartDiscovery method
        fn start_discovery(&self) -> zbus::Result<()>;
    
        /// StopDiscovery method
        fn stop_discovery(&self) -> zbus::Result<()>;
    
        /// Address property
        #[zbus(property)]
        fn address(&self) -> zbus::Result<String>;
    
        /// AddressType property
        #[zbus(property)]
        fn address_type(&self) -> zbus::Result<String>;
    
        /// Alias property
        #[zbus(property)]
        fn alias(&self) -> zbus::Result<String>;
        #[zbus(property)]
        fn set_alias(&self, value: &str) -> zbus::Result<()>;
    
        /// Class property
        #[zbus(property)]
        fn class(&self) -> zbus::Result<u32>;
    
        /// Discoverable property
        #[zbus(property)]
        fn discoverable(&self) -> zbus::Result<bool>;
        #[zbus(property)]
        fn set_discoverable(&self, value: bool) -> zbus::Result<()>;
    
        /// DiscoverableTimeout property
        #[zbus(property)]
        fn discoverable_timeout(&self) -> zbus::Result<u32>;
        #[zbus(property)]
        fn set_discoverable_timeout(&self, value: u32) -> zbus::Result<()>;
    
        /// Discovering property
        #[zbus(property)]
        fn discovering(&self) -> zbus::Result<bool>;
    
        /// ExperimentalFeatures property
        #[zbus(property)]
        fn experimental_features(&self) -> zbus::Result<Vec<String>>;
    
        /// Modalias property
        #[zbus(property)]
        fn modalias(&self) -> zbus::Result<String>;
    
        /// Name property
        #[zbus(property)]
        fn name(&self) -> zbus::Result<String>;
    
        /// Pairable property
        #[zbus(property)]
        fn pairable(&self) -> zbus::Result<bool>;
        #[zbus(property)]
        fn set_pairable(&self, value: bool) -> zbus::Result<()>;
    
        /// PairableTimeout property
        #[zbus(property)]
        fn pairable_timeout(&self) -> zbus::Result<u32>;
        #[zbus(property)]
        fn set_pairable_timeout(&self, value: u32) -> zbus::Result<()>;
    
        /// Powered property
        #[zbus(property)]
        fn powered(&self) -> zbus::Result<bool>;
        #[zbus(property)]
        fn set_powered(&self, value: bool) -> zbus::Result<()>;
    
        /// Roles property
        #[zbus(property)]
        fn roles(&self) -> zbus::Result<Vec<String>>;
    
        /// UUIDs property
        #[zbus(property, name = "UUIDs")]
        fn uuids(&self) -> zbus::Result<Vec<String>>;
    }
    
    #[proxy(
        interface = "org.bluez.AgentManager1",
        default_service = "org.bluez",
        default_path = "/org/bluez"
    )]
    pub trait AgentManager {
        /// RegisterAgent method
        fn register_agent(
            &self,
            agent: &zbus::zvariant::ObjectPath<'_>,
            capability: &str,
        ) -> zbus::Result<()>;
    
        /// RequestDefaultAgent method
        fn request_default_agent(&self, agent: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;
    
        /// UnregisterAgent method
        fn unregister_agent(&self, agent: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;
    }
    
    #[proxy(interface = "org.bluez.Device1", default_service = "org.bluez")]
    pub trait Device {
        /// CancelPairing method
        fn cancel_pairing(&self) -> zbus::Result<()>;
    
        /// Connect method
        fn connect(&self) -> zbus::Result<()>;
    
        /// ConnectProfile method
        fn connect_profile(&self, uuid: &str) -> zbus::Result<()>;
    
        /// Disconnect method
        fn disconnect(&self) -> zbus::Result<()>;
    
        /// DisconnectProfile method
        fn disconnect_profile(&self, uuid: &str) -> zbus::Result<()>;
    
        /// Pair method
        fn pair(&self) -> zbus::Result<()>;
    
        /// Adapter property
        #[zbus(property)]
        fn adapter(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
    
        /// Address property
        #[zbus(property)]
        fn address(&self) -> zbus::Result<String>;
    
        /// AddressType property
        #[zbus(property)]
        fn address_type(&self) -> zbus::Result<String>;
    
        /// Alias property
        #[zbus(property)]
        fn alias(&self) -> zbus::Result<String>;
        #[zbus(property)]
        fn set_alias(&self, value: &str) -> zbus::Result<()>;
    
        /// Appearance property
        #[zbus(property)]
        fn appearance(&self) -> zbus::Result<u16>;
    
        /// Blocked property
        #[zbus(property)]
        fn blocked(&self) -> zbus::Result<bool>;
        #[zbus(property)]
        fn set_blocked(&self, value: bool) -> zbus::Result<()>;
    
        /// Bonded property
        #[zbus(property)]
        fn bonded(&self) -> zbus::Result<bool>;
    
        /// Class property
        #[zbus(property)]
        fn class(&self) -> zbus::Result<u32>;
    
        /// Connected property
        #[zbus(property)]
        fn connected(&self) -> zbus::Result<bool>;
    
        /// Icon property
        #[zbus(property)]
        fn icon(&self) -> zbus::Result<String>;
    
        /// LegacyPairing property
        #[zbus(property)]
        fn legacy_pairing(&self) -> zbus::Result<bool>;
    
        /// ManufacturerData property
        #[zbus(property)]
        fn manufacturer_data(
            &self,
        ) -> zbus::Result<std::collections::HashMap<u16, zbus::zvariant::OwnedValue>>;
    
        /// Modalias property
        #[zbus(property)]
        fn modalias(&self) -> zbus::Result<String>;
    
        /// Name property
        #[zbus(property)]
        fn name(&self) -> zbus::Result<String>;
    
        /// Paired property
        #[zbus(property)]
        fn paired(&self) -> zbus::Result<bool>;
    
        /// RSSI property
        #[zbus(property, name = "RSSI")]
        fn rssi(&self) -> zbus::Result<i16>;
    
        /// ServiceData property
        #[zbus(property)]
        fn service_data(
            &self,
        ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;
    
        /// ServicesResolved property
        #[zbus(property)]
        fn services_resolved(&self) -> zbus::Result<bool>;
    
        /// Trusted property
        #[zbus(property)]
        fn trusted(&self) -> zbus::Result<bool>;
        #[zbus(property)]
        fn set_trusted(&self, value: bool) -> zbus::Result<()>;
    
        /// TxPower property
        #[zbus(property)]
        fn tx_power(&self) -> zbus::Result<i16>;
    
        /// UUIDs property
        #[zbus(property, name = "UUIDs")]
        fn uuids(&self) -> zbus::Result<Vec<String>>;
    
        /// WakeAllowed property
        #[zbus(property)]
        fn wake_allowed(&self) -> zbus::Result<bool>;
        #[zbus(property)]
        fn set_wake_allowed(&self, value: bool) -> zbus::Result<()>;
    }
    
    #[proxy(
        interface = "org.bluez.GattManager1",
        default_service = "org.bluez",
        default_path = "/org/bluez/hci0"
    )]
    trait GattManager {
        /// RegisterApplication method
        fn register_application(
            &self,
            application: &zbus::zvariant::ObjectPath<'_>,
            options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        ) -> zbus::Result<()>;
    
        /// UnregisterApplication method
        fn unregister_application(
            &self,
            application: &zbus::zvariant::ObjectPath<'_>,
        ) -> zbus::Result<()>;
    }
    
    #[proxy(
        interface = "org.bluez.HealthManager1",
        default_service = "org.bluez",
        default_path = "/org/bluez"
    )]
    trait HealthManager {
        /// CreateApplication method
        fn create_application(
            &self,
            config: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
    
        /// DestroyApplication method
        fn destroy_application(&self, application: &zbus::zvariant::ObjectPath<'_>)
            -> zbus::Result<()>;
    }
    
    #[proxy(
        interface = "org.bluez.LEAdvertisingManager1",
        default_service = "org.bluez",
        default_path = "/org/bluez/hci0"
    )]
    trait LEAdvertisingManager {
        /// RegisterAdvertisement method
        fn register_advertisement(
            &self,
            advertisement: &zbus::zvariant::ObjectPath<'_>,
            options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        ) -> zbus::Result<()>;
    
        /// UnregisterAdvertisement method
        fn unregister_advertisement(
            &self,
            service: &zbus::zvariant::ObjectPath<'_>,
        ) -> zbus::Result<()>;
    
        /// ActiveInstances property
        #[zbus(property)]
        fn active_instances(&self) -> zbus::Result<u8>;
    
        /// SupportedIncludes property
        #[zbus(property)]
        fn supported_includes(&self) -> zbus::Result<Vec<String>>;
    
        /// SupportedInstances property
        #[zbus(property)]
        fn supported_instances(&self) -> zbus::Result<u8>;
    
        /// SupportedSecondaryChannels property
        #[zbus(property)]
        fn supported_secondary_channels(&self) -> zbus::Result<Vec<String>>;
    }
    
    #[proxy(
        interface = "org.bluez.Media1",
        default_service = "org.bluez",
        default_path = "/org/bluez/hci0"
    )]
    trait Media {
        /// RegisterApplication method
        fn register_application(
            &self,
            application: &zbus::zvariant::ObjectPath<'_>,
            options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        ) -> zbus::Result<()>;
    
        /// RegisterEndpoint method
        fn register_endpoint(
            &self,
            endpoint: &zbus::zvariant::ObjectPath<'_>,
            properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        ) -> zbus::Result<()>;
    
        /// RegisterPlayer method
        fn register_player(
            &self,
            player: &zbus::zvariant::ObjectPath<'_>,
            properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        ) -> zbus::Result<()>;
    
        /// UnregisterApplication method
        fn unregister_application(
            &self,
            application: &zbus::zvariant::ObjectPath<'_>,
        ) -> zbus::Result<()>;
    
        /// UnregisterEndpoint method
        fn unregister_endpoint(&self, endpoint: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;
    
        /// UnregisterPlayer method
        fn unregister_player(&self, player: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;
    
        /// SupportedUUIDs property
        #[zbus(property, name = "SupportedUUIDs")]
        fn supported_uuids(&self) -> zbus::Result<Vec<String>>;
    }
    
    #[proxy(
        interface = "org.bluez.NetworkServer1",
        default_service = "org.bluez",
        default_path = "/org/bluez/hci0"
    )]
    trait NetworkServer {
        /// Register method
        fn register(&self, uuid: &str, bridge: &str) -> zbus::Result<()>;
    
        /// Unregister method
        fn unregister(&self, uuid: &str) -> zbus::Result<()>;
    }
    
    #[proxy(
        interface = "org.bluez.ProfileManager1",
        default_service = "org.bluez",
        default_path = "/org/bluez"
    )]
    pub trait ProfileManager {
        /// RegisterProfile method
        fn register_profile(
            &self,
            profile: &zbus::zvariant::ObjectPath<'_>,
            uuid: &str,
            options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        ) -> zbus::Result<()>;
    
        /// UnregisterProfile method
        fn unregister_profile(&self, profile: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;
    }
}
pub mod proxy {
    pub use super::interface::AdapterProxy;
    pub use super::interface::DeviceProxy;
    pub use super::interface::AgentManagerProxy;
    pub use super::interface::ProfileManagerProxy;

}
