/*
 * OS Utilities
 */

use std::fs::File;
use std::io::Read;
use std::time::Duration;
use std::thread;

use common::{Error, Result};

struct Stats {
    user: f32,
    nice: f32,
    system: f32,
    idle: f32
}

/*
 * Given two stats separated by n number of milliseconds,
 * calculate the current CPU usage
 */
fn cpu_usage_percent(s1: Stats, s2: Stats) -> f32 {
    let user = s2.user - s1.user;
    let nice = s2.nice - s1.nice;
    let system = s2.system - s1.system;
    let idle = s2.idle - s1.idle;

    return ((user + nice + system) / idle) * 100.0;
}

/*
 * Read /proc/stat into a Stats struct
 */
fn global_cpu_stats() -> Result<Stats> {
    let mut f = try!(File::open("/proc/stat"));
    let mut s = String::new();

    try!(f.read_to_string(&mut s));

    let line = try!(s.lines().next().ok_or(Error::new("Invalid /proc/stat: no lines")));
    let mut parts = line.split_whitespace();

    try!(parts.next().ok_or(Error::new("Invalid /proc/stat: mising cpu value")));

    let cpu_user = try!(try!(parts.next().ok_or(Error::new("Invalid /proc/stat: mising user value")))
        .parse::<f32>().ok().ok_or(Error::new("Invalid /proc/stat number")));

    let cpu_nice = try!(try!(parts.next().ok_or(Error::new("Invalid /proc/stat: mising nice value")))
        .parse::<f32>().ok().ok_or(Error::new("Invalid /proc/stat number")));

    let cpu_system = try!(try!(parts.next().ok_or(Error::new("Invalid /proc/stat: mising system value")))
        .parse::<f32>().ok().ok_or(Error::new("Invalid /proc/stat number")));

    let cpu_idle = try!(try!(parts.next().ok_or(Error::new("Invalid /proc/stat: mising idle value")))
        .parse::<f32>().ok().ok_or(Error::new("Invalid /proc/stat number")));

    Ok(Stats {
        user: cpu_user,
        nice: cpu_nice,
        system: cpu_system,
        idle: cpu_idle
    })
}

/*
 * Return the system's global CPU usage in percent
 */
pub fn global_cpu_usage() -> Result<f32> {
    let s1 = try!(global_cpu_stats());
    thread::sleep(Duration::from_millis(250));
    let s2 = try!(global_cpu_stats());

    Ok(cpu_usage_percent(s1, s2))
}


pub fn global_memory_info() -> Result<(f32, f32)> {
    let mut mem_total = 0.0;
    let mut mem_free = 0.0;
    let mut mem_buffers = 0.0;
    let mut mem_cached = 0.0;

    // Open /proc/meminfo and read it as a string
    let mut f = try!(File::open("/proc/meminfo"));
    let mut s = String::new();

    try!(f.read_to_string(&mut s));

    // Gather memory information
    for line in s.lines() {
        let mut parts = line.split_whitespace();
        let key = try!(parts.next().ok_or(Error::new("Invalid /proc/meminfo")));
        let val = try!(parts.next().ok_or(Error::new("Invalid /proc/meminfo")));

        if key == "MemTotal:" {
            mem_total = try!(val.parse::<f32>().ok().ok_or(Error::new("Invalid /proc/meminfo number")));
        }
        if key == "MemFree:" {
            mem_free = try!(val.parse::<f32>().ok().ok_or(Error::new("Invalid /proc/meminfo number")));
        }
        if key == "Buffers:" {
            mem_buffers = try!(val.parse::<f32>().ok().ok_or(Error::new("Invalid /proc/meminfo number")));
        }
        if key == "Cached:" {
            mem_cached = try!(val.parse::<f32>().ok().ok_or(Error::new("Invalid /proc/meminfo number")));
        }
    }

    Ok(((mem_total - (mem_free + mem_buffers + mem_cached)) / 1024.0, mem_total / 1024.0))
}
