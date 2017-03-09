/*
 * Network module - Networking utilities (bridge, TAP, DHCP, ebtables...)
 */

pub mod dhcp;

use regex::Regex;

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
pub fn network_bridge(name: &str) -> String {
    format!("net{}", name)
}
