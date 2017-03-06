#!/bin/bash

err() {
	echo "$@" 1>&2;
	exit 1
}

if [[ $# -ne 1 ]]; then
	err "Usage: create.sh <json def>"
fi

name=$(echo $1 | jq -r .name)
image=$(echo $1 | jq -r .image)
folder="/var/lib/olvm/vm/$name"
disk="$folder/disk.qcow2"

if [[ ! -d $folder ]]; then
	mkdir -p $folder
fi

output=""

if [[ ! $image == "null" ]]; then
	base=$(echo $1 | jq -r .image.file)
	output=$(qemu-img create -f qcow2 -b "$base" "$disk" 15G)
else
	output=$(qemu-img create -f qcow2 "$disk" 15G)
fi

if [[ $output == *"Formatting"* ]]; then
	exit 0
else
	err "Failed to create disk"
fi
