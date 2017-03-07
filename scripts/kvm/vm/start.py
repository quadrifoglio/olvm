#!/usr/bin/python2

import sys
import time
import json
import subprocess
import os

vm = json.loads(sys.argv[1])
folder = '/var/lib/olvm/vm/' + vm['name']
disk = folder + '/disk.qcow2'

child = subprocess.Popen(["/usr/bin/qemu-system-x86_64", "-nographic"],
    stdout = subprocess.PIPE,
    stderr = subprocess.PIPE
)

print "pid", child.pid
sys.exit(0)
