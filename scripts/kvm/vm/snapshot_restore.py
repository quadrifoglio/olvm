#!/usr/bin/python2

import sys
import time
import json
import subprocess

snap = json.loads(sys.argv[1])
name = snap["name"]
vm = snap["vm"]
folder = '/var/lib/olvm/vms/kvm/' + vm['name']
