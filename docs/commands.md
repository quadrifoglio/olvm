# Commands

## Image

### JSON representation

```
{
	"name": string - required - primary key,
	"backend": string - required - name of the backend to use (kvm, openvz...),
	"file": string - required - path to the actual image file,

	"parameters": { - optional
		"key": "value",
		...
	}
}
```

### createimg

Create a new image. Requires a JSON image argument.

### listimg

List all the images

### getimg

Get the JSON representation of the specified image.

Parameter: name (string) - name of the image

### updateimg

Update an existing image. Requires a JSON image argument.

### delimg

Delete the specified image

Parameter: name (string) - name of the image

## Virtual Machines

### JSON representation

```
{
	"name": string - required - primary key,
	"backend": string - required - name of the backend to use (kvm, openvz...),

	"image": string - optional - name of an image to base the VM on (if any),
	"parameters": { - optional
		"key": "value",
		...
	}
}
```

### createvm

Create a new VM. Requires a JSON VM argument.

### listvm

List all the VMs

### getvm

Get the JSON representation of the specified VM.

Parameter: name (string) - name of the VM

### updatevm

Update an existing VM. Requires a JSON VM argument.

### delvm

Delete the specified VM

Parameter: name (string) - name of the VM

### startvm

Start the specified VM

Parameter: name (string) - name of the VM

### stopvm

Stop the specified VM

Parameter: name (string) - name of the VM
