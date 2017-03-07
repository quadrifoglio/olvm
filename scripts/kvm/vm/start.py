#!/usr/bin/python2

import sys
import time
import json
import subprocess

vm = json.loads(sys.argv[1])
params = vm['parameters']

folder = '/var/lib/olvm/vm/' + vm['name']
disk = folder + '/disk.qcow2'

opts = [
    '/usr/bin/qemu-system-x86_64',
    '-nographic',
    '-drive', 'format=qcow2,file=' + disk
]

if 'acceleration' in params:
    opts.append('-enable-kvm')

if 'cpus' in params:
    opts.append('-smp')
    opts.append(params['cpus'])

if 'memory' in params:
    opts.append('-m')
    opts.append(params['memory'])

if 'vnc' in params:
    opts.append('-vnc')

    if 'vncws' in params:
        opts.append(params['vnc'] + ',websocket=' + params['vncws'])
    else:
        opts.append(params['vnc'])

if 'args' in params:
    opts.append(params['args'])

child = subprocess.Popen(opts, stdout = subprocess.PIPE, stderr = subprocess.PIPE)

time.sleep(2)
child.poll()

if child.returncode is None:
    print 'pid', child.pid
    sys.exit(0)
else:
    stdout, stderr = child.communicate()

    sys.stderr.write('qemu exited: ' + stderr)
    sys.exit(1)
