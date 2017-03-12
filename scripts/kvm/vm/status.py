#!/usr/bin/python2

import sys
import time
import json
import os

vm = json.loads(sys.argv[1])
params = vm['parameters']

if not 'pid' in params or params['pid'] == '0':
    print 'running false'
    sys.exit(0)

pid = int(params['pid'])

try:
    os.kill(pid, 0)
    print 'running true'
except OSError:
    print 'running false'

sys.exit(0)
