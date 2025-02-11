# Seawolf 8 CUDA Efforts
## Testing CUDA on a fresh system
### Steps Taken
- Use disk image from NVIDIA Jetson Nano developer kit: [image](https://developer.nvidia.com/jetson-nano-sd-card-image)
- Upgrade the system: `apt upgrade` (Should not be necessary)
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
## Cross Compiling sw8s_rust for test jetson
This is an attempt at using the current code and tooling to compile sw8s_rust for test Jetson.
### Steps Taken
#### 1. sysroot creation
+ Mount the test Jetson's SD card
+ Make a copy of [Marcus' sysroot script](https://github.com/MB3hel/RustCrossExperiments/blob/76933201f80aec397bc37eadfcdbaacac5da109e/make-sysroot.sh)
+ Create a sysroot for use with the [cross compiler tooling](https://github.com/ncsurobotics/SW8S-Rust/tree/ff5172924862d9723de956318dd68b89e7734327/jetson):
```
# This assumes that:
#	- The SD card is already mounted at $JETSON_MNT
#	- make-sysroot.sh is in the cwd
#	- $SYSROOT_DIR is an empty directory
# make-sysroot.sh copies file from $JETSON_MNT to $SYSROOT_DIR

sudo ./make-sysroot.sh $JETSON_MNT $SYSROOT_DIR
cd $SYSROOT_DIR
sudo tar -cvf sysroot-jetson.tar *
xz -z -T0 -v sysroot-jetson.tar
```
#### 2. Compiling with new sysroot
+ Clone the [SW8S-Rust](https://github.com/ncsurobotics/SW8S-Rust) repo
+ Unpack the sysroot archive to a directory named `sysroot-jetson` in the root of the repo
+ Modify `./jetson/src/main.rs` (relative to repo root) to match updated library paths (TODO: Document changes)
+ Force override the opencv crate to its latest version (this is probably an issue)
+ Fix errors in our code base related to upstream opencv API changes
+ Run the cross compiler as normal
### Outcome
- The steps above allow for successfully building the binary
- Attempting to run the binary on the Jetson will result in an error that an opencv .so ending in 406 cannot be found
	- This suggests that something in the code is still trying to rely on the old version of opencv
	- A .so with the same base name exists in the sysroot, but it ends in 412 (version 4.12)