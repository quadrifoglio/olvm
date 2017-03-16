#!/usr/bin/python2

import lxc
import json
import sys

vm = json.loads(sys.argv[1])

c = lxc.Container(vm['name'])
if c.defined:
    sys.stderr.write('vm already exists')
    sys.exit(1)

if not c.create(vm['image']['name']):
    sys.stderr.write('Failed to create the container rootfs')
    sys.exit(1)
