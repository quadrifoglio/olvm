#!/usr/bin/python2

import sys
import time
import json
import subprocess

vm = json.loads(sys.argv[1])
folder = '/var/lib/olvm/vm/' + vm['name']
disk = folder + '/disk.qcow2'

try:
    ps = subprocess.Popen(['qemu-system-x86_64', '-nographic', '-drive', 'file='+disk],
        stdin = subprocess.PIPE,
        stdout = subprocess.PIPE
    )

    time.sleep(2)
    ps.poll()

    if ps.returncode == None:
        print "pid", ps.pid
    else:
        sys.stderr.write("qemu-system-x86_64 has exited with status " + str(ps.returncode))

except subprocess.CalledProcessError:
    sys.exit(1)
