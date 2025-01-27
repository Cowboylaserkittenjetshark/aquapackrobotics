## Notes
- Overshoots final barrel roll
	- Nose kicks up
	- We may be able to fix this by cutting the final roll early
- Front IO caps have cables that fall into the front camera
- Bottom camera inconsistantly working
	- We should try with a different bottom cam
	- Front camera consistently works
- Use the old version of the AUV control board repo. The latest version does not work
- The buoy detection seems to be inconsistent with the lighting inside Carmichael
	- Both buoys were tried
	- Real buoy seemed to perform slightly better, but not enough to warrant the effort
- Torpedo servos were twitching without code runing
## Current camera test procedures
```
# Managing the rtsp server
## This should be auto started on login as of now
systemctl --user start rtsp-simple-server   # Start the server
systemctl --user status rtsp-simple-server  # Get status of the server
systemctl --user restart rtsp-simple-server # Restart the server

# Starting cameras
## with seawolf disarmed:
~/deploy/sw8s_rust arm
```
Cameras should now be accesible at `rtsp://192.168.2.5:8554/{front, bottom}.mp4`.
If they are not, check the status of the rtsp server.
