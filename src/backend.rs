/*
 * Backend - Actual hypervisor
 */

use error::{Result, Error};

/*
 * Return the ID corresponding to the specified backend
 */
pub fn from_str(s: &str) -> Result<i32> {
    match s {
        "kvm" | "KVM" | "Kvm" => Ok(1),
        _ => Err(Error::new("Invalid backend"))
    }
}

/*
 * Return the string representation of a backend
 * base on its ID
 */
pub fn to_string(i: i32) -> Result<String> {
    match i {
        1 => Ok("kvm".to_string()),
        _ => Err(Error::new("Invalid backend"))
    }
}
