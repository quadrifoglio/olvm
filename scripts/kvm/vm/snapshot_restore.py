#!/usr/bin/python2

import sys
import json
import socket

snap = json.loads(sys.argv[1])
name = snap["name"]
vm = snap["vm"]
folder = '/var/lib/olvm/vms/kvm/' + vm['name']
monitor = folder + '/monitor.sock'

if 'pid' not in vm['parameters'] or vm['parameters']['pid'] == '0':
    sys.stderr.write('The VM is not running')
    sys.exit(1)

sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)

try:
    sock.connect(monitor)
except IOError as e:
    sys.stderr.write(e.strerror)
    sys.exit(1)

data = '{"execute": "qmp_capabilities"}'
sock.send(data)
sock.recv(1024)

data = '{"execute": "human-monitor-command", "arguments": {"command-line": "loadvm ' + name + '"}}'
sock.send(data)

data = sock.recv(1024)

if '"return": {}' not in data:
    sys.stderr.write('return from qmp: ' + data)
    sys.exit(1)
