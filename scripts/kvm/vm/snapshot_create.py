#!/usr/bin/python2

import sys
import time
import json
import socket

snap = json.loads(sys.argv[1])
name = snap["name"]
vm = snap["vm"]
folder = '/var/lib/olvm/vms/kvm/' + vm['name']
monitor = folder + '/monitor.sock'

sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)

try:
    sock.connect(monitor)
except IOError as e:
    sys.stderr.write(e.strerror)
    sys.exit(1)

data = '{"execute": "qmp_capabilities"}'
sock.send(data)

data = '{"execute": "human-monitor-command", "arguments": {"command-line": "savevm' + name + '"}}'
sock.send(data)

data = socket.recv(1024)
sys.stderr.write(data)
