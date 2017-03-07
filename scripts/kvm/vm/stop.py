#!/usr/bin/python2

import sys
import os
import signal
import json

vm = json.loads(sys.argv[1])

if not 'parameters' in vm or not 'pid' in vm['parameters'] or vm['parameters']['pid'] == 0:
    sys.exit(0)

pid = int(vm['parameters']['pid'])

try:
    os.kill(pid, signal.SIGTERM)
finally:
    print "pid", 0
