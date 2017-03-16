#!/usr/bin/python2

import sys
import json
import subprocess

img = json.loads(sys.argv[1])

try:
    subprocess.check_output(['rm', '/usr/share/lxc/templates/lxc-' + img['name']])
except subprocess.CalledProcessError as e:
    sys.stderr.write('Failed to remove template: ' + e.output)
    sys.exit(1)
