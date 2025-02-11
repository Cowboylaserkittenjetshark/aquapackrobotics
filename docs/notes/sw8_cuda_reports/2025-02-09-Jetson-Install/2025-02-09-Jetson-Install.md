# CUDA capable Jetson Install
## 1. Install L4T
1. [Get the official image from NVIDIA](https://developer.nvidia.com/jetson-nano-sd-card-image)
2. Plug in SD card and find out what device it has enumerated as: `lsblk`
3. Flash downloaded image to SD card: `unzip -p /home/cblkjs/Downloads/jetson-nano-jp461-sd-card-image.zip | sudo dd of=/dev/sda bs=1M status=progress`
4. Once dd has finished, unplug the SD card and put it in a Jetson
5. Perform initial setup for the jetson
## 2. Setup swap space for OpenCV build
```bash
sudo apt-get update
sudo apt-get upgrade
sudo apt-get install dphys-swapfile

# Set: 
#   `CONF_SWAPSIZE=4096`
#   `CONF_MAXSWAP=4096`
sudo vim /etc/dphys-swapfile
sudo dphys-swapfile setup
sudo dphys-swapfile swapon

sudo reboot
free -m # Verify total swap space is ~6000
```
## 3. Run Qengineering OpenCV install script
```bash
cd ~
git clone https://github.com/Qengineering/Install-OpenCV-Jetson-Nano.git
cd Install-OpenCV-Jetson-Nano
chmod +x OpenCV-4-11-0.sh
./OpenCV-4-11-0.sh
cd ~
rm -rf Install-OpenCV-Jetson-Nano
```
## 4. Check OpenCV installation
```bash
python3
```
```python
import cv2
cv2.__version__
exit()
```
## 5. Cleanup
```bash
sudo /etc/init.d/dphys-swapfile stop
sudo apt-get remove --purge dphys-swapfile
sudo rm /var/swap
sudo rm -rf ~/opencv
sudo rm -rf ~/opencv_contrib
```
## 6. Setup networking
### Option A (Preferred): Use `systemd-networkd`
```bash
sudo systemctl disable wpa_supplicant.service
sudo systemctl disable NetworkManager.service
sudo systemctl disable dnsmasq.service

sudo vim /etc/wpa_supplicant/wpa_supplicant-wlan0.conf
# Add the following to /etc/wpa_supplicant/wpa_supplicant-wlan0.conf:
# ctrl_interface=/var/run/wpa_supplicant
# update_config=1
# network={
#     ssid="ncsu"
#     key_mgmt=NONE
# }

sudo vim /etc/systemd/network/20-wireless.network
# Add the following to /etc/systemd/network/20-wireless.network:
# [Match]
# Name=wlan0
# [Network]
# DHCP=yes
# IgnoreCarrrierLoss=3s

sudo vim /etc/systemd/network/10-wired-tether.network
# Add the following to /etc/systemd/network/10-wired-tether.network
# [Match]
# Name=eth0
# [Network]
# Address=192.168.2.5
# DHCPServer=yes

sudo systemctl enable systemd-networkd.service
sudo systemctl enable wpa_supplicant@wlan0.service
reboot
```
Wireless networks other than `ncsu` can be configured using the `wpa_cli` command
### Option B (The old setup): dnsmasq
These instructions are actually broken and are only here for historical reasons. Use option A instead, it is faster and more robust. For dnsmasq to autostart, `eth0` needs to be given a static ip using ifconfig. 
```bash
# Set static IP through NetworkManager
nmcli con add con-name "static-eth0" ifname eth0 type ethernet ip4 192.168.2.5/24# Add the following to /etc/dnsmasq.d/sw8-eth0.conf
nmcli con del Wired\ connection\ 1
nmcli con up static-eth0

# It may be neccesary to add 

# Setup dnsmasq as a DHCP server
sudo apt-get install dnsmasq
sudo vim /etc/dnsmasq.d/sw8-eth0.conf
# Add the following to /etc/dnsmasq.d/sw8-eth0.conf:
# domain=local
# interface=eth0
# dhcp-range=192.168.2.15,192.168.2.100,255.255.255.0,24h
```
## 7. Setup RTSP server
```bash
# Download mediamtx
mkdir ~/mediamtx
cd ~/mediamtx
wget https://github.com/bluenviron/mediamtx/releases/download/v1.11.3/mediamtx_v1.11.3_linux_arm64v8.tar.gz
tar -xvf mediamtx_v1.11.3_linux_arm64v8.tar.gz
rm mediamtx_v1.11.3_linux_arm64v8.tar.gz

# Setup systemd user service
vim ~/.config/systemd/user/mediamtx.service
# The file should contain the following:
# [Service]
# ExecStart=%h/mediamtx/mediamtx %h/mediamtx/mediamtx.yml
# Restart=on-failure
# RestartSec=5
# 
# [Unit]
# Wants=network.target
# Description=mediamtx
# 
# [Install]
# WantedBy=default.target
systemctl --user daemon-reload
systemctl --user enable --now mediamtx.service

# Install RTSP plugin for gstreamer
sudo apt-get install gstreamer1.0-rtsp
```
![[mediamtx.service]]
## 8. Setup for our workflow
```bash 
cd ~

# AUVControlBoard scripts
git clone https://github.com/ncsurobotics/AUVControlBoard.git
cd AUVControlBoard
git checkout v1.0.3-beta0 # We need this version, main is broken
python3 -m pip install pyserial # Scripts depend on this

mkdir ~/deploy

# Needed to communicate with control board and meb
sudo usermod -aG dialout sw8
reboot
```