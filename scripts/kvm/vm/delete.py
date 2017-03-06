#!/usr/bin/python2

import json
import os.path
import sys
import shutil

vm = json.loads(sys.argv[1])
folder = '/var/lib/olvm/vm/' + vm['name']

if os.path.isdir(folder):
    shutil.rmtree(folder)
