#!/usr/bin/python2

import sys
import json
import subprocess

snap = json.loads(sys.argv[1])
name = snap['name']
vm = snap['vm']
sys.stderr.write(vm['name'])
folder = '/var/lib/olvm/vms/kvm/' + vm['name']
disk = folder + '/disk.data'

try:
    subprocess.check_output(["qemu-img", "snapshot", "-d", name, disk])
except CalledProcessError as e:
    sys.stderr.write(e)
    sys.exit(1)
