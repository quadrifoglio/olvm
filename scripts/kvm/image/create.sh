#!/bin/bash

err() {
	echo "$@" 1>&2;
	exit 1
}

if [[ $# -ne 1 ]]; then
	err "Usage: create.sh <json def>"
fi

json=$1
file=$(echo $json | jq -r '.file')
output=$(qemu-img check $file 2>&1)

if [[ ! $output == *"No errors were found on the image"* ]]; then
	err "The specified image contains errors"
fi

chk=$(sha1sum $file | awk '{print $1}')
echo "sha1 $chk"
