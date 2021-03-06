/*
 * Network System - OS actions to manage networking
 */

use std::process::Command;

use common::{Result, Error};

/*
 * Create a bridge interface
 */
pub fn bridge_create(name: &str) -> Result<()> {
    let exists = try!(Command::new("ip").arg("link").arg("show").arg(name).output());
    if !exists.status.success() {
        let out = try!(Command::new("ip")
            .arg("link").arg("add").arg(name)
            .arg("type").arg("bridge").output());

        if !out.status.success() {
            let err = match String::from_utf8(out.stderr) {
                Ok(err) => err,
                Err(_) => return Err(Error::new("Failed to read 'ip' output as a string"))
            };

            return Err(Error::new(format!("Failed to create bridge: {}", err)));
        }
    }

    let up = try!(Command::new("ip").arg("link").arg("set").arg("up").arg("dev").arg(name).output());
    if !up.status.success() {
        let err = match String::from_utf8(up.stderr) {
            Ok(err) => err,
            Err(_) => return Err(Error::new("Failed to read 'ip' output as a string"))
        };

        return Err(Error::new(format!("Failed to set up bridge: {}", err)));
    }

    Ok(())
}

/*
 * Add a network interface to a bridge
 */
pub fn bridge_addif(iface: &str, bridge: &str) -> Result<()> {
    let out = try!(Command::new("ip")
        .arg("link").arg("set").arg("master").arg(bridge)
        .arg("dev").arg(iface).output());

    if !out.status.success() {
        let err = match String::from_utf8(out.stderr) {
            Ok(err) => err,
            Err(_) => return Err(Error::new("Failed to read 'ip' output as a string"))
        };

        return Err(Error::new(format!("Failed to add '{}' to '{}' bridge: {}", iface, bridge, err)));
    }

    Ok(())
}

/*
 * Delete a bridge interface
 */
pub fn bridge_delete(name: &str) -> Result<()> {
    let out = try!(Command::new("ip").arg("link").arg("del").arg(name).output());

    if !out.status.success() {
        let err = match String::from_utf8(out.stderr) {
            Ok(err) => err,
            Err(_) => return Err(Error::new("Failed to read 'ip' output as a string"))
        };

        return Err(Error::new(format!("Failed to delete bridge: {}", err)));
    }

    Ok(())
}

/*
 * Create a TAP interface
 */
pub fn tap_create(name: &str) -> Result<()> {
    let exists = try!(Command::new("ip").arg("tuntap").arg("show").arg(name).output());

    let stdout = match String::from_utf8(exists.stdout) {
        Ok(stdout) => stdout,
        Err(_) => return Err(Error::new("Failed to read stdout as a string"))
    };

    if !exists.status.success() || !stdout.contains(name) {
        let out = try!(Command::new("ip")
            .arg("tuntap").arg("add").arg(name)
            .arg("mode").arg("tap").output());

        if !out.status.success() {
            let err = match String::from_utf8(out.stderr) {
                Ok(err) => err,
                Err(_) => return Err(Error::new("Failed to read 'ip' output as a string"))
            };

            return Err(Error::new(format!("Failed to create TAP: {}", err)));
        }
    }

    Ok(())
}

/*
 * Delete a TAP interface
 */
pub fn tap_delete(name: &str) -> Result<()> {
    let out = try!(Command::new("ip")
        .arg("tuntap").arg("del").arg(name)
        .arg("mode").arg("tap").output());

    if !out.status.success() {
        let err = match String::from_utf8(out.stderr) {
            Ok(err) => err,
            Err(_) => return Err(Error::new("Failed to read 'ip' output as a string"))
        };

        return Err(Error::new(format!("Failed to delete TAP: {}", err)));
    }

    Ok(())
}
