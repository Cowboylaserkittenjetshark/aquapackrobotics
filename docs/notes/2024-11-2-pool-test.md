# Pool Test - November 2nd, 2024
## Notes
### Video feed setup
- RTSP Server on seawolf
- sw8s_rust program publishes mp4 file for the client
- RTSP client on laptop


### dnsmasq
The jetson is running dnsmasq as a DHCP server

## Summary
### Missions
- depth_test: worked as expected
- spin: spun (3 times plus a half spin)
- Full run: Kinda sorta worked as expected
  - First run: Wolf went towards the left side of the gate and went to the outside before being killed
  - The second run, we aimed it at the right side of the gate and it went through the gate as expected. 
    - After making it through the gate, it stalled waiting for something (buoy?)
    - Did not do barrel rolls as a result

### Problems
- Some conditions (idk what. repluging cameras? Enumeration issues?) may cause gstreamer to be unable to open a stream from the cameras
- Rebooting the jetson is not enough to reset the cameras (power to the USB ports may not be fully cut)
- A full reboot with the systems switch in neccesary
- Setting up deterministic enumeration should fix this
  - This is not as easy as setting up UDEV rules unfortunately
  - Each camera needs a unique serial number: https://docs.arducam.com/UVC-Camera/Serial-Number-Tool-Guide/
- **A log file in the ~/deploy directory on the jetson should have the errors**
- We recompiled sw8s_rust and copied it to the jetson without making any changes to the source code.
  - It seems like the timing of this and the camera issues starting lines up, but we could not find any reason why this would be the cause
  - A full reboot with the systems switch *did* fix the issues temporarily, so the recompilation is likely unrelated
