#!/usr/bin/python2

import sys
import json
import subprocess

vm = json.loads(sys.argv[1])
folder = '/var/lib/olvm/vm/' + vm['name']
disk = folder + '/disk.qcow2'

try:
    out = ''

    if 'image' in vm:
        out = subprocess.check_output(['qemu-img', 'create', '-b', vm['image']['file'], img['file'], '15G'])
    else
        out = subprocess.check_output(['qemu-img', 'create', img['file'], '15G'])

    if 'Formatting' not in out:
        sys.stderr.write('qemu-img create failed')
        sys.exit(1)

except subprocess.CalledProcessError:
    sys.exit(1)
