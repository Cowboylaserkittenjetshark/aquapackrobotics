This details the process I went through to cross compile the CUDA example that we compiled natively on the jetson in the last report.

**Outline:**
1. Building and fixing the sysroot
2. Modify the cross compile tool in the SW8S-Rust repo
3. Update the SW8S-Rust repo to a more recent version of the opencv crate
4. Enable extra features for the opencv crate
5. Build the example
## 1. Building and fixing the sysroot
Marcus' original `make-sysroot.sh` script that was used in the last report does not generate a valid sysroot for our installation of Linux for Tegra:
- It incorrectly assumes that `/lib` is a symlink to `/usr/lib`
	- On our test system, several symlinks in `/usr/lib` point to files in `/lib`. This means the original script will break these symlinks
- It does not copy the `/etc/alternatives` directory, which contains the targets of several symlinks in `/usr/lib` and `/lib`
- It fails to convert all absolute symlinks to relative ones

This version is a quick but messy fix that includes `/lib` and `/etc/alternatives`. It does not fix the problem with converting symlinks from absolute to relative. 
```bash
#!/usr/bin/env bash
set -e

if [ "$(id -u)" -ne 0 ]; then
    echo "Run as root!"
    exit 1
fi

# Parse arguments
if [ $# -ne 2 ]; then
    echo "Usage: $0 root_dir dest_folder"
    echo "Makes a sysroot folder from a root folder (mounted image, debootstrap, etc)."
    echo ""
    echo "Example:"
    echo "  $0 /mnt/jetsonroot $HOME/sysroot-jetson"
    echo ""
    exit 1
fi

# Make sure image file exists
rootdir="$1"
if [ ! -d "$rootdir" ]; then
    echo "Root directory does not exist."
    exit 1
fi

# Get absolute path with no trailing slash
# Important for how links are fixed
destdir="$(realpath "$2")"
destdir="${destdir%"/"}"
if [ ! -d "$destdir" ]; then
    echo "Destination folder does not exist."
    exit 1
fi

# Copy files matching certain patterns
echo "Copying files to sysroot directory"
rsync -a \
    --exclude=bin \
    --exclude=sbin \
    --exclude=src \
    --exclude=share \
    --exclude=libexec \
    --exclude=games \
    --exclude=lib/aarch64-linux-gnu/dri \
    --exclude=lib/firmware \
    --exclude=local/cuda-10.2/doc \
    --exclude=local/cuda-10.2/samples \
    --exclude=lib/systemd \
    "$rootdir/usr/" "$destdir/usr/"
  rsync -a \
    --exclude=aarch64-linux-gnu/dri \
    --exclude=firmware \
    --exclude=systemd \
    "$rootdir/lib/" "$destdir/lib/"
rsync -a "$rootdir/opt/" "$destdir/opt/"
mkdir -p "$destdir/etc/"
rsync -a "$rootdir/etc/alternatives" "$destdir/etc/"
echo ""

# Convert links to all be relative (absolute links won't work in porable sysroot)
echo "Fixing links in sysroot"
while read l; do
    if [ -z "$l" ]; then
        # Empty string -> find probably found nothing?
        continue
    fi

    # Directory containing the link
    ldir=$(dirname "$l")

    # Absolute target of link
    tabs=$(readlink "$l")

    # Absolute target in sysroot directory
    tabsnew="$destdir/$tabs"

    # Relative target in sysroot directory
    trelnew="$(realpath -m --relative-to="$ldir" "$tabsnew")"

    # Replace link with relative link
    ln -sf "$trelnew" "$l"
done <<< "$(find "$destdir" -type l -lname '/*')"
echo ""

# Finally, check for any broken links
# This is mostly useful in determining if anything important was missed previously
# This doesn't fix broken links. Just prints them so user knows if script
# needs to be modified to fix them
echo "Checking for broken links"
while read l; do
    if [ ! -L "$l" ] || [ ! -e "$l" ]; then
        t=$(readlink "$l")
        echo "Found broken link in sysroot: $l -> $t"
    fi
done <<< "$(find "$destdir" -type l)"
echo ""

# Done. No cleanup necessary.
echo "Sysroot created."

```

I manually converted the remaining absolute symlinks that the script missed.
## 2. Modify the cross compile tool
All of the paths in the current version of the cross compile tool are incorrect for the install on the test jetson. My updated version is located [here](https://github.com/Cowboylaserkittenjetshark/SW8S-Rust/blob/e306c68466d611c878795d2adf5ee9e8ad3a641c/jetson/src/main.rs). This tries to link too many libraries, but does work and does not seem to affect the size of the executable.

There was also a line that assumed all files ending in `.so` would begin with `lib`, which is not actually the case. This resulted in the tool trying to pass `ld-2.27.so` as `-l2.27`, which should not happen. This was fixed.
## 3. Update the opencv crate
6. Manaully edit `Cargo.toml` and change the version of the opencv crate to the latest version. 
7. Run `cargo update`
## 4. Enable extra features
The example requires the `cudafilters` and `cudaimgproc` features of the opencv crate to be enabled. Add these to the appropriate features array in `Cargo.toml`.

## 5. Build the example
Replace the contents of the `src/main.rs` file in the SW8S-Rust repo with [the example](https://github.com/twistedfall/opencv-rust/blob/97dcbd0e4a897c9d77204d3d5c2edacfeb4a5805/examples/cuda.rs), then follow the regular procedures for cross compiling.

This should fail, but we can now use the error messages to figure out what libraries we need to fix the symlinks for. There are also some symlinks that do exist that need to be removed so that they are not linked. This process is mostly trial and error plus google-fu.