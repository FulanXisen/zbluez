use std::collections::HashMap;
use std::error::Error;
use std::ffi::CString;
use std::io::BufRead;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use std::result::Result;
use tokio::io::AsyncBufReadExt;
use tokio::time::sleep;
use tokio_stream::StreamExt;
use zbus::zvariant::{self, OwnedObjectPath};
use zbus::{zvariant::Value, Connection};

mod bluez;
use bluez::{AdapterProxy, DeviceProxy};
use zbus::fdo;
use zbus::fdo::{InterfacesAdded, InterfacesAddedArgs, InterfacesAddedStream};
use zbus::fdo::{InterfacesRemoved, InterfacesRemovedArgs, InterfacesRemovedStream};
use zbus::fdo::{ManagedObjects, ObjectManagerProxy};
use zbus::Error::FDO;

use libc::{close, pipe};

use tokio::fs::File;
// #[proxy(
//     interface = "org.bluez.Adapter1",
//     default_service = "org.bluez",
//     default_path = "/org/bluez/hci0"
// )]
// trait BluezAdapter {
//     #[zbus(property)]
//     fn address(&self) -> Result<String, zbus::Error>;
//     #[zbus(property)]
//     fn discovering(&self) -> Result<bool, zbus::Error>;
//     #[zbus(property)]
//     fn powered(&self) -> Result<bool, zbus::Error>;
//     #[zbus(property)]
//     fn set_powered(&self, power: bool) -> Result<(), zbus::Error>;

//     #[zbus(signal)]
//     fn device_found(
//         &self,
//         address: String,
//         values: HashMap<String, zvariant::OwnedValue>,
//     ) -> zbus::Result<()>;
//     #[zbus(signal)]
//     fn device_disappeared(&self, address: String) -> zbus::Result<()>;
//     #[zbus(signal)]
//     fn device_created(&self, path: String) -> zbus::Result<()>;
//     #[zbus(signal)]
//     fn device_removed(&self, path: String) -> zbus::Result<()>;
//     // StartDiscovery method
//     fn start_discovery(&self) -> Result<(), zbus::Error>;
//     // StopDiscovery method
//     fn stop_discovery(&self) -> Result<(), zbus::Error>;
//     // SetDiscoveryFilter
//     //fn set_discovery_filter(&self, array_of_dict: Vec<HashMap<String, zvariant::OwnedValue>>);
// }

struct BluetoothDeviceManager {
    system_connection: Connection,
    adapter_proxy: AdapterProxy<'static>,
    object_manager_proxy: ObjectManagerProxy<'static>,

    classic_connected: Vec<String>,
    le_connected: Vec<String>,
    classic_connecting: Vec<String>,
    le_connecting: Vec<String>,
    connected: Vec<String>,
    connecting: Vec<String>,
    connect_result_receiver: mpsc::Receiver<(String, &'static str, zbus::Result<()>)>,
    connect_result_sender: mpsc::Sender<(String, &'static str, zbus::Result<()>)>,
}
fn address_to_path(addr: &str) -> String {
    format!("/org/bluez/hci0/dev_{}", addr.replace(":", "_"))
}

impl BluetoothDeviceManager {
    async fn get_device_proxy(&self, address: String) -> zbus::Result<DeviceProxy> {
        let objects: ManagedObjects = self.object_manager_proxy.get_managed_objects().await?;
        let found = objects.keys().find(|&path| path.ends_with(&address));
        match found {
            Some(path) => {
                let obj = objects.get(path).unwrap();
                Ok(DeviceProxy::new(&self.system_connection, address_to_path(&address)).await?)
            }
            None => Err(zbus::Error::FDO(Box::new(zbus::fdo::Error::Failed(
                String::from("Object Path Not Managed"),
            )))),
        }
    }

    fn should_start_discovering(&self) -> bool {
        todo!()
    }
    fn start_discovery(&self) {}
    fn should_stop_discovering(&self) -> bool {
        todo!()
    }
    fn stop_discovery(&self) {}
    fn ensure_connected(&self, address: String) -> bool {
        todo!()
    }
    fn classic_connect(&self, address: String) -> bool {
        if self.should_start_discovering() {
            self.start_discovery();
        }

        if self.should_stop_discovering() {}
        todo!()
    }
    fn le_connect(&self, address: String) -> bool {
        todo!()
    }
    fn main_loop(&mut self) -> zbus::Result<()> {
        loop {
            match self.connect_result_receiver.recv() {
                Ok((address, class, result)) => {
                    if result.is_ok() {
                        if class == "Classic" {
                            self.classic_connecting
                                .retain(|addr| !addr.eq_ignore_ascii_case(&address));
                            self.classic_connected.push(address);
                        } else {
                            self.le_connected
                                .retain(|addr| !addr.eq_ignore_ascii_case(&address));
                            self.le_connected.push(address);
                        }
                    } else {
                        eprintln!(
                            "Device {} {} connect failed with {:?}",
                            address, class, result
                        );
                        eprintln!("Device {} {} connect retry", address, class);
                    }
                }
                Err(e) => {
                    println!("receiver recv() failed with {}", e);
                }
            }
        }
    }
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

// Although we use `async-std` here, you can use any async runtime of choice.
async fn do_main() -> Result<(), Box<dyn Error>> {
    let connection = Connection::system().await?;

    let proxy = AdapterProxy::new(&connection).await?;
    // let properties_proxy = BluezPropertiesProxy::new(&connection).await?;
    println!("Address: {}", proxy.address().await?);
    println!("Discovering: {}", proxy.discovering().await?);
    let power = proxy.powered().await?;
    println!("Powered: {}", power);
    if power == false {
        let _ = proxy.set_powered(true).await?;
    }
    sleep(Duration::from_millis(100)).await;
    println!("Powered: {}", proxy.powered().await?);

    proxy.start_discovery().await?;
    sleep(Duration::from_millis(100)).await;
    println!("Discovering: {}", proxy.discovering().await?);
    //let mut properties_changed = properties_proxy.receive_properties_changed().await?;

    //sleep(Duration::from_millis(2000)).await;

    // let mut new_device_found = proxy.receive_device_found().await?;
    // while let Some(msg) = new_device_found.next().await {
    //     let args: DeviceFoundArgs = msg.args().expect("Error parsing message");
    //     println!("Device Found: {:?}", args);
    // }
    // println!("changed: {:?}", properties_changed);
    // while let Some(msg) = properties_changed.next().await {
    //     let args: PropertiesChangedArgs = msg.args().expect("Error parsing message");
    //     println!(
    //         "Properties Changed: p0={:?} p1={:?} p2={:?}",
    //         args.p0, args.p1,args.p2
    //     )
    // }
    let object_manager_proxy = ObjectManagerProxy::new(&connection, "org.bluez", "/").await?;
    let mut interfaces_added_signal_receiver =
        object_manager_proxy.receive_interfaces_added().await?;
    let mut interfaces_removed_signal_receiver =
        object_manager_proxy.receive_interfaces_removed().await?;
    //println!("{:?}", object_manager_signal_receiver);
    let interface_added_handle = tokio::spawn(async move {
        interface_added_signal_cb(&mut interfaces_added_signal_receiver).await;
    });
    let interface_removed_handle = tokio::spawn(async move {
        interface_removed_signal_cb(&mut interfaces_removed_signal_receiver).await;
    });

    sleep(Duration::from_millis(5000)).await;
    proxy.stop_discovery().await?;
    sleep(Duration::from_millis(60000)).await;

    let interface_added_abort_handle = interface_added_handle.abort_handle();
    let interface_removed_abort_handle = interface_removed_handle.abort_handle();
    interface_added_abort_handle.abort();
    interface_removed_abort_handle.abort();
    assert!(interface_added_handle.await.unwrap_err().is_cancelled());
    assert!(interface_removed_handle.await.unwrap_err().is_cancelled());

    Ok(())
}

type Address = String;
enum Cmd {
    StartDiscovering,
    StopDiscovering,
    ConnectClassic(Address),
    ConnectLE(Address),
    DisconnectClassic(Address),
    DisconnectLE(Address),
    PlayMediaA2DP(Address, u32),
    PlayMediaHFP(Address, u32),
    SendGattCmd(Vec<u8>),
    Remove(Address),
}
use std::{env, fs, io};
const REQ_PIPE_VAR_NAME: &str = "ZBLUEZ_REQ_PIPE";
const RESP_PIPE_VAR_NAME: &str = "ZBLUEZ_RESP_PIPE";

async fn open_pipe_from_env(name: &str, out: bool) -> tokio::io::Result<File> {
    match env::var(name) {
        Ok(file_path) => {
            let path = Path::new(&file_path);
            if path.exists() {
                dbg!("path exists");
                match fs::remove_file(path) {
                    Ok(_) => println!("Delete existed pipe"),
                    Err(e) => {
                        println!("Error deleting pipe: {:?} {:?}", path, e);
                    }
                }
            } else {
                dbg!("path not exists");
            }
            dbg!("checking CString");
            let cpath = CString::new(file_path.bytes().collect::<Vec<u8>>())?;
            dbg!("checking mkfifo");
            match unsafe {
                libc::mkfifo(
                    cpath.as_ptr(),
                    libc::S_IRUSR | libc::S_IWUSR | libc::S_IRGRP | libc::S_IROTH,
                )
            } {
                0 => {
                    println!("Creating pipe: {:?}", path);
                    if out {
                            Ok(tokio::fs::OpenOptions::new()
                                .custom_flags(libc::O_NONBLOCK)
                                .read(true)
                                .open(path)
                                .await?)
                        // ret = Some(
                        //     tokio::fs::OpenOptions::new()
                        //         .custom_flags(libc::O_NONBLOCK)
                        //         .write(true)
                        //         .open(path)
                        //         .await?,
                        // );
                    } else {
                        Ok(
                            tokio::fs::OpenOptions::new()
                                .read(true)
                                .custom_flags(libc::O_NONBLOCK)
                                .open(path)
                                .await?
                        )
                    }
                }
                c => {
                    Err(tokio::io::Error::from_raw_os_error(c))
                }
            }
        }
        Err(e) => {
            Err(tokio::io::Error::last_os_error())
        }
    }
}

async fn config() -> tokio::io::Result<(File,File)> {
    let req_pipe = open_pipe_from_env(REQ_PIPE_VAR_NAME, false).await?;
    let resp_pipe = open_pipe_from_env(RESP_PIPE_VAR_NAME, true).await?;
    Ok((req_pipe, resp_pipe))
}

async fn listen(req_pipe: File) -> tokio::io::Result<()> {
    let mut reader = tokio::io::BufReader::new(req_pipe);
    let mut cmd = String::new();
    while let Ok(n) = reader. read_line(&mut cmd).await {
        if n == 0 {
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
            println!("skip");
            continue;
        }
        print!("cmd: {}", cmd);
        cmd.clear();
    }
    
    println!("loop ...");
    std::thread::sleep(Duration::from_secs(1));
    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //let (req_pipe, resp_pipe) = config();
    let req_pipe = open_pipe_from_env(REQ_PIPE_VAR_NAME, false).await?;
    Ok(listen(req_pipe).await?)
}
