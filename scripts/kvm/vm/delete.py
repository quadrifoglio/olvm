#!/usr/bin/python2

import json
import os.path
import sys

vm = json.loads(sys.argv[1])
folder = '/var/lib/olvm/vm/' + vm['name']

if os.path.isdir(folder):
    os.rmdir(folder)
