#!/usr/bin/python2

import sys
import json
import subprocess

img = json.loads(sys.argv[1])

try:
    out = subprocess.check_output(['qemu-img', 'check', img["file"]])

    if "No errors were found on the image" not in out:
        sys.stderr.write('qemu-img check failed')
        sys.exit(1)

except subprocess.CalledProcessError:
    sys.exit(1)
