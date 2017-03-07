# KVM Backend Documentation

## Optional parameters

### Hardware Acceleration

Name: 'acceleration'

Possible values: do not matter at all

When present, this parameters enables the hardware-assisted virtualization (Intel-VT or AMD-V).
This is of course due to the KVM kernel module.

### Number of CPU cores

Name: 'cpus'

Possible values: any integer (up to 255)

Simulate a multi-core guest VM.
Set this to the number of CPU cores you want to make available to the guest.

### RAM

Name: 'memory'

Possible values: any integer

Set the VM's available memory.
Default is 128MiB.
Supported suffixes: M (MiB), or G (GiB)

### VNC

Name: 'vnc'

Possible values: a string to represent a bind adress (ip:display, such as '127.0.0.1:1')

Create a VNC server to access the guest's display and input system.
Choose a bind address and a display number (port).

### VNC over WebSocket

Name: 'vncws'

Possible values: an integer (display number)

Available only when the VNC server is listening (see above).
Choose a WebSocket VNC display number (port).

### Custom command line options

Name: 'args'

Possible values: any string

This parameter will be apended to the end of the qemu/kvm command line.
You can use this to set custom options that are not supported by this backend.
