/*
 * Network module - Networking utilities (bridge, TAP, DHCP, ebtables...)
 */

pub mod system;
pub mod dhcp;

use std::sync::Arc;

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
        for iface in vm.interfaces {
            let tap = iface_dev(vm.name.as_str(), index);
            try!(system::tap_create(tap.as_str()));

            index = index + 1;
        }
    }

    dhcp::listen(ctx)
}

/*
 * Check is the specified string is a valid IP address
 */
pub fn is_valid_ip(ip: &str) -> bool {
   Regex::new(r"^([0-9]{0:3})\.([0-9]{0:3})\.([0-9]{0:3})\.([0-9]{0:3})$").unwrap().is_match(ip)
}

/*
 * Check if the specified string is a valid CIDR network address
 */
pub fn is_valid_cidr(cidr: &str) -> bool {
    let index = match cidr.find('/') {
        Some(index) => index,
        None => return false
    };

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
