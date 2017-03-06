#!/usr/bin/python2

import sys
import json
import subprocess
import os
import os.path

vm = json.loads(sys.argv[1])
folder = '/var/lib/olvm/vm/' + vm['name']
disk = folder + '/disk.qcow2'

if not os.path.isdir(folder):
    os.makedirs(folder)

try:
    out = ''

    if 'image' in vm:
        out = subprocess.check_output(['qemu-img', 'create', '-f', 'qcow2', '-b', vm['image']['file'], disk, '15G'])
    else:
        out = subprocess.check_output(['qemu-img', 'create', '-f', 'qcow2', disk, '15G'])

    if 'Formatting' not in out:
        sys.stderr.write('qemu-img create failed')
        sys.exit(1)

except subprocess.CalledProcessError:
    sys.exit(1)
