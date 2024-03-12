use tokio_stream::StreamExt;
use zbus::zvariant::{self};
use zbus::Connection;

//use crate::simple_bluez::proxy::{AdapterProxy,DeviceProxy};
use zbus::fdo::ObjectManagerProxy;
use zbus::fdo::{InterfacesAddedArgs, InterfacesAddedStream};
use zbus::fdo::{InterfacesRemovedArgs, InterfacesRemovedStream};

use crate::simple_bluez::proxy::{AdapterProxy, DeviceProxy};

pub struct BluetoothServer {
    conn: Connection,
    iface_added_handle: Box<tokio::task::JoinHandle<()>>,
    iface_removed_handle: Box<tokio::task::JoinHandle<()>>,
}

impl BluetoothServer {
    pub async fn new() -> zbus::Result<Self> {
        let conn = Connection::system().await?;
        let object_manager_proxy = ObjectManagerProxy::new(&conn, "org.bluez", "/").await?;
        let mut interfaces_added_iter = object_manager_proxy.receive_interfaces_added().await?;
        let mut interfaces_removed_iter = object_manager_proxy.receive_interfaces_removed().await?;
        //println!("{:?}", object_manager_signal_receiver);
        let interface_added_handle = tokio::spawn(async move {
            interface_added_signal_cb(&mut interfaces_added_iter).await;
        });
        let interface_removed_handle = tokio::spawn(async move {
            interface_removed_signal_cb(&mut interfaces_removed_iter).await;
        });
        Ok(BluetoothServer {
            conn: conn,
            iface_added_handle: Box::new(interface_added_handle),
            iface_removed_handle: Box::new(interface_removed_handle),
        })
    }

    pub async fn get_adapter(&self) -> zbus::Result<AdapterProxy> {
        AdapterProxy::new(&self.conn).await
    }

    pub async fn get_objects_manager(&self) -> zbus::Result<ObjectManagerProxy> {
        ObjectManagerProxy::new(&self.conn, "org.bluez", "/").await
    }

    pub async fn get_device(&self, address: &str) -> zbus::Result<DeviceProxy> {
        let object_path = zvariant::OwnedObjectPath::try_from(address_to_object_path(address))?;
        let objects = self
            .get_objects_manager()
            .await?
            .get_managed_objects()
            .await?;
        let found = objects.keys().find(|&path| path == &object_path);
        match found {
            Some(path) => {
                let obj = objects.get(path).unwrap();
                Ok(DeviceProxy::new(&self.conn, path.clone()).await?)
            }
            None => Err(zbus::Error::Failure(format!(
                "ObjectPath of {} not found",
                object_path.to_string()
            ))),
        }
    }

    pub async fn cleanup(&self) {
        self.iface_added_handle.abort_handle().abort();
        self.iface_removed_handle.abort_handle().abort();
        //println!("abort thread: {}", self.iface_added_handle.await.unwrap_err().is_cancelled());
        //println!("abort thread: {}", self.iface_added_handle.await.unwrap_err().is_cancelled());
    }
}

fn address_to_object_path(addr: &str) -> String {
    format!("/org/bluez/hci0/dev_{}", addr.replace(":", "_"))
}

async fn interface_added_signal_cb<'a>(stream: &mut InterfacesAddedStream<'_>) {
    while let Some(msg) = stream.next().await {
        let args: InterfacesAddedArgs = msg.args().expect("Error parsing message");
        let path = args.object_path;
        let content = args.interfaces_and_properties;
        println!("=========={}==========", path);

        for (_, v) in content {
            for (ki, vi) in v {
                println!("Added {} : {:?}", ki, vi);
            }
        }
    }
}

async fn interface_removed_signal_cb<'a>(stream: &mut InterfacesRemovedStream<'_>) {
    while let Some(msg) = stream.next().await {
        let args: InterfacesRemovedArgs = msg.args().expect("Error parsing message");
        let path = args.object_path;
        let content = args.interfaces;
        println!("=========={}==========", path);

        for v in content {
            println!("Removed {}", v);
        }
    }
}
