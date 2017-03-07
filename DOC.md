# OLVM - Open Linux Virtual Machines

This simple software can be used as a way to manage virtual machines
running on multiple backend hypervisors easily.

## Usage

Check the configuration file.
By default, this software listens for commands on a UDP socket.

### Commands

A command is a single-word string representing an action, such as 'createimg' or 'startvm'.

Some commands require an argument, which can be either:

* a JSON representation of an object, for example the object to be created
* a string representing the name of an object

Checkout docs/commands.md for a complete list of commands.
