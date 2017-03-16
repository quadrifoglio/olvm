#!/usr/bin/python2

import sys
import json
import subprocess

img = json.loads(sys.argv[1])

try:
    subprocess.check_output(['cp', img['file'], '/usr/share/lxc/templates/lxc-' + img['name']])
except subprocess.CalledProcessError as e:
    sys.stderr.write('Failed to copy template: ' + e.output)
    sys.exit(1)

try:
    subprocess.check_output(['chmod', '+x', '/usr/share/lxc/templates/lxc-' + img['name']])
except subprocess.CalledProcessError as e:
    sys.stderr.write('Failed to chmod template: ' + e.output)
    sys.exit(1)
