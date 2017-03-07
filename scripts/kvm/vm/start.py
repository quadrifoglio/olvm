#!/usr/bin/python2

import sys
import time
import json
import subprocess
import os

vm = json.loads(sys.argv[1])
folder = '/var/lib/olvm/vm/' + vm['name']
disk = folder + '/disk.qcow2'

child = subprocess.Popen(['/usr/bin/qemu-system-x86_64', '-nographic'],
    stdout = subprocess.PIPE,
    stderr = subprocess.PIPE
)

time.sleep(2)
child.poll()

if child.returncode is None:
    print 'pid', child.pid
    sys.exit(0)
else:
    stdout, stderr = child.communicate()

    sys.stderr.write('qemu exited: ' + stderr)
    sys.exit(1)
