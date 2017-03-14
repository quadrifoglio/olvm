/*
 * Network module - Networking utilities (bridge, TAP, DHCP, ebtables...)
 */

pub mod system;
pub mod dhcp;

use std::sync::Arc;

use uuid::{Uuid, UuidVersion};
use regex::Regex;

use common::{Context, Result};
use database;

/*
 * Setup the networking module: create network interfaces, start DHCP server
 */
pub fn setup(ctx: Arc<Context>) -> Result<()> {
    let nets = try!(database::network::list(&ctx.db));

    for net in nets {
        let netdev = net_dev(net.name.as_str());
        try!(system::bridge_create(netdev.as_str()));
    }

    let vms = try!(database::vm::list(&ctx.db));

    for vm in vms {
        let mut index = 0;
        for _ in vm.interfaces {
            let tap = iface_dev(vm.name.as_str(), index);
            try!(system::tap_create(tap.as_str()));

            index = index + 1;
        }
    }

    dhcp::listen(ctx)
}

/*
 * Generate a random MAC address
 */
pub fn rand_mac() -> String {
    let u = Uuid::new(UuidVersion::Random).unwrap();
    let bytes = u.as_bytes();

    format!("52:54:01:{:02x}:{:02x}:{:02x}", bytes[0], bytes[1], bytes[2])
}

/*
 * Check is the specified string is a valid IP address
 */
pub fn is_valid_ip(ip: &str) -> bool {
   Regex::new(r"^([0-9]{0,3})\.([0-9]{0,3})\.([0-9]{0,3})\.([0-9]{0,3})$").unwrap().is_match(ip)
}

/*
 * Check if the specified string is a valid CIDR network address
 */
pub fn is_valid_cidr(cidr: &str) -> bool {
    let index = match cidr.find('/') {
        Some(index) => index,
        None => return false
    };

    println!("{}", &cidr[..index]);

    is_valid_ip(&cidr[..index])
}

/*
 * Returns the bridge interface name corresponding to a network name
 */
pub fn net_dev(name: &str) -> String {
    format!("net{}", name)
}

/*
 * Returns the interface name corresponding to a VM's nth network interface
 */
pub fn iface_dev(mut vm: &str, index: i32) -> String {
    if vm.len() > 10 {
        vm = &vm[..10];
    }

    format!("vm{}.{}", vm, index)
}
