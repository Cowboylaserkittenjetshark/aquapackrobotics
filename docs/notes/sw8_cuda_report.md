# Seawolf 8 CUDA Efforts
## Testing CUDA on a fresh system
### Steps Taken
- Use disk image from NVIDIA Jetson Nano developer kit: [image](https://developer.nvidia.com/jetson-nano-sd-card-image)
- Upgrade the system: `apt upgrade` (Should not be neccesary)
- Install alternate version of opencv (4.1.1 -> 4.12 or 4.11)
  - https://qengineering.eu/install-opencv-on-jetson-nano.html
  - https://github.com/Qengineering/Install-OpenCV-Jetson-Nano
- Run the CUDA example from the opencv crate
  - Clone the [opencv-rust repo](https://github.com/twistedfall/opencv-rust/)
  - Get a random image (the png/jpeg kind) to use for testing
  - `cargo run --example cuda /path/to/random/image`
### Outcome
- The example indicates that CUDA is available and working:
```
Device 0:  "NVIDIA Tegra X1"  3964Mb, sm_53, Driver/Runtime ver.10.20/10.20
CUDA is available
Timing CPU implementation... 
2.438369789s
Timing CUDA implementation... 
6.446543329s
```
- `jtop` indicates that:
	- The GPU is being utilized by the example program
	- (Press 7) The OpenCV version that we installed earlier does support CUDA
- All of these tests were done directly on the Jetson. We attempted to reuse the crosscompile program in the sw8s_rust repo and the compiled binary did not work (could not load libopencv_dnn.so.406 when run on the jetson)
	- This may be fixable by feeding the crosscompile program a sysroot image built from our current Jetson install