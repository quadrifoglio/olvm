#!/usr/bin/python2

import json
import os.path
import sys
import shutil

vm = json.loads(sys.argv[1])
f = '/var/lib/olvm/images/kvm/' + vm['name'] + '.image'

if os.path.exists(f):
    os.remove(f)
