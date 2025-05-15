# MEB Troubleshooting
## Port
First, double check that [everything is connected to the Jetson](./dryrun.md#verify-everything-is-connected-to-the-jetson).

If everything is actually connected, then MEB has a different name than what is expected. We need to determine this name and use it instead.

> [!IMPORTANT]
> You should inform the software team as soon as possible if the device name format has actually changed. This is unexpected and needs to be investigated.

To determine the name, run the following command:
```bash
ls /dev/serial/by-id/ | grep -i Texas
```

This command lists files in the `/dev/serial/by-id/` folder that contain the word Texas (case insensitive) in their name. It should output 2 file names. If it does not output anything, then MEB is in fact not connected to the Jetson. The output should look similar to this:
```bash
/dev/serial/by-id/usb-Texas_Instruments_MSP_Tools_Driver_B20A826e14002300-if00	
/dev/serial/by-id/usb-Texas_Instruments_MSP_Tools_Driver_B20A826e14002300-if02	
```

Now, we will take these resulting files names and determine which one is the correct one.

Starting with the first file name, do the following:
1. Run the following commands, replacing FILE_NAME with the file name we are testing:
	```bash
	export PORT=$(realpath FILE_NAME)
	echo $PORT
	```
2. Begin following the steps in [MEB Communication & Voltage Monitor](./dryrun.md#meb-communication--voltage-monitor)
	 1. If you encounter no errors, you have selected the correct one and can continue
	 2. If you encounter any errors, return here and start from step 1 using the second file name
	 3. If you encounter errors with both file names, reboot the robot and try again
	 4. If the issue persist, stop and ask for assistance.
