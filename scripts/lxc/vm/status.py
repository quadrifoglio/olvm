#!/usr/bin/python2

import lxc
import json
import sys

vm = json.loads(sys.argv[1])

c = lxc.Container(vm['name'])
if not c.defined:
    sys.stderr.write('vm does not exists')
    sys.exit(1)

if c.running:
    print 'running true'
else:
    print 'running false'
