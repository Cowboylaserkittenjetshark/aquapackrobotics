# Jetson Troubleshooting
## Checking Connection
> [!IMPORTANT]
> Most connection problems can be resolved by waiting for a few minutes and retrying.
>
> However, you should never have to wait longer than 3 minutes. If you do, continue to the troubleshooting steps below.

To establish an [SSH connection](https://simple.wikipedia.org/wiki/Secure_Shell) to the Jetson, two important things must happen:
1. The computer must obtain an [IP address](https://simple.wikipedia.org/wiki/IP_address) from the [DHCP server](https://simple.wikipedia.org/wiki/Dynamic_Host_Configuration_Protocol) running on the Jetson
2. The SSH server running on the Jetson must be available to receive connections from the computer

### 1. Checking IP 
#### Linux (Team laptop)
When using the team laptop or another computer running [Linux](https://simple.wikipedia.org/wiki/Linux), follow these instructions:
1. Disconnect the robot from the computer
2. If using an [ethernet](https://simple.wikipedia.org/wiki/Ethernet) [dongle](https://simple.wikipedia.org/wiki/Dongle), disconnect if from the computer
3. Open a new terminal
4. Run the following command:
   ```bash
   ip a
   ```

   You should see output that looks similar to the following. Comments (lines starting with "#") have been added for explanation.
   ```bash
   # This is the first entry (notice it start with "1:")
   1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN group default qlen 1000
       link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
       inet 127.0.0.1/8 scope host lo
          valid_lft forever preferred_lft forever
       inet6 ::1/128 scope host noprefixroute 
          valid_lft forever preferred_lft forever
      
   # This is the second entry (it start with a "2:")
   # The "enp4s0" is the name of one of the computer's network interfaces (https://simple.wikipedia.org/wiki/Network_card)
   # By naming convention, "enp4s0" starts with "enp" and is therefore a wired network interface
   # Notice the "state DOWN" here. This means that this network interface is not connected to anything
   2: enp4s0: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc fq_codel state DOWN group default qlen 1000
       link/ether b0:25:aa:3f:f4:24 brd ff:ff:ff:ff:ff:ff
       altname enxb025aa3ff424
    
   # This is the third entry (it start with a "3:")
   # By naming convention, "wlan0" is a wireless network interface because it starts with "wlan"
   # This is the interface my computer uses to make a WiFi connection
   # This interface has "state UP", meaning that it is connected to a network
   # Notice that this entry has a line that begins with "inet"
   #   The "192.168.86.24/24" indicates the ip address that your computer has on the network
   3: wlan0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default qlen 1000
       link/ether 70:9c:d1:fa:44:de brd ff:ff:ff:ff:ff:ff
       inet 192.168.86.24/24 metric 1024 brd 192.168.86.255 scope global dynamic wlan0
          valid_lft 85906sec preferred_lft 85906sec
       inet6 fe80::729c:d1ff:fefa:44de/64 scope link proto kernel_ll 
          valid_lft forever preferred_lft forever
   # There may more entries depending on the system
   ```
5. If using a ethernet dongle, connect it to the computer
6. Connect the robot to the computer
7. Run `ip a` again
   * If you are using an ethernet dongle, you should now see a new entry in the output (the dongle acts as a network interface)
   
     The interface should now have "state UP" and have an "inet" line
     ```bash
     10: enp195s0f4u1: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP group default qlen 1000
        link/ether 56:66:c9:8b:0f:92 brd ff:ff:ff:ff:ff:ff
        altname enx5666c98b0f92
        inet 192.168.82.177/24 metric 1024 brd 192.168.82.255 scope global dynamic enp195s0f4u1
           valid_lft 3590sec preferred_lft 3590sec
        inet6 fe80::5466:c9ff:fe8b:f92/64 scope link proto kernel_ll 
           valid_lft forever preferred_lft forever
     ```
   * If you are connecting the robot to the computer using your computers built-in ethernet port, the "enp" interface from before should now have "state UP" and have an "inet" line
8. If the interface is up (has "state UP") and is assigned an ip address (has "inet" line), continue to [SSH](#ssh). Otherwise, wait roughly 10 seconds and repeat step 7. If, after a few tries, you are still not assigned an ip address, continue to [Trouble Shooting IP Issues](#trouble-shooting-ip-issues)

#### Windows
> [!WARNING]
> The team laptop is known to work, unless you know what you are doing, please use it instead
>
> The following steps have not been tested

1. Disconnect the robot from the computer
2. If using an [ethernet](https://simple.wikipedia.org/wiki/Ethernet) [dongle](https://simple.wikipedia.org/wiki/Dongle), disconnect if from the computer
3. Open a new terminal
4. Run the command `gip`
5. If using and ethernet dongle, connect it to the computer
6. Connect the robot to the computer
7. Run the `gip` command again
8. Check if you have been assigned an ip address:
   * If using an ethernet dongle, a new network interface should have appeared
   * If connecting directly to the computer's ethernet port, the ethernet interface should now have an ip address ("IPv4Address") 
9. If you have been assigned an ip address, continue to [SSH](#ssh). Otherwise, wait roughly 10 seconds and repeat step 8. If, after a few tries, you are still not assigned an ip address, continue to [Trouble Shooting IP Issues](#trouble-shooting-ip-issues)

#### Trouble Shooting IP Issues
- If the interface remains down, the Jetson likely does not have power. If you have confirmed that it is powered on, there is likely a physical break in the connection between the computer and the Jetson.
- If the interface is up but does not have an ip address, the DHCP server may be misbehaving. Power cycle the robot. If the issue continues, you should ask a member of the software team.
- If you are using an ethernet dongle and the new interface does not appear, try a different one.

### SSH
The following instructions should apply to both Linux and Windows.

1. Run the following command in a terminal:
   ```bash
   ssh sw8@192.168.2.5
   ```
2. You should almost immediately be prompted about a fingerprint or for a password. If so, everything is fine and you should continue dryrunning.
3. Otherwise, if you receive a connection refused error, the SSH server is likely still starting
   1. Wait at most two minutes
   2. Rerun `ssh sw8@192.168.2.5`
   3. If the error persists, reboot the robot
   4. If the error persists even after rebooting, ask a software team member
4. Otherwise, if the command hangs with no output or you receive a connection timed out error:
   1. Kill the ssh command if running (press Ctrl and C at the same time)
   2. Wait at most 2 minutes
   3. Rerun`ssh sw8@192.168.2.5`
   4. If the problem persists, reboot the robot
   5. If the problem persists even after rebooting, kill the ssh command and run:
      ```bash
      ping 192.168.2.5
      ```
   6. Let it run for about 5 seconds, then kill it (press Ctrl and C at the same time)

      This is just a diagnostic step, at this point you will likely need to ask a software team member regardless of the result.

      The output should look like the following if the Jetson is up:
      ```bash
      PING 192.168.2.5 (192.168.2.5) 56(84) bytes of data.
      64 bytes from 192.168.2.5: icmp_seq=1 ttl=57 time=48.3 ms
      64 bytes from 192.168.2.5: icmp_seq=2 ttl=57 time=148 ms
      64 bytes from 192.168.2.5: icmp_seq=3 ttl=57 time=62.6 ms
      64 bytes from 192.168.2.5: icmp_seq=4 ttl=57 time=50.9 ms
      64 bytes from 192.168.2.5: icmp_seq=5 ttl=57 time=70.2 ms
      64 bytes from 192.168.2.5: icmp_seq=6 ttl=57 time=48.5 ms
      ^C
      --- 192.168.2.5 ping statistics ---
      6 packets transmitted, 6 received, 0% packet loss, time 5010ms
      rtt min/avg/max/mdev = 48.338/71.487/148.450/35.342 ms.5: icmp_seq=6 ttl=57 time=48.5 ms
      ```
      This means that you can communicate with the Jetson, but that SSH server is misbehaving.
      
      If the Jetson is unreachable or not responding, it should look like this:
      ```bash
      PING 192.168.2.5 (192.168.2.5) 56(84) bytes of data.
      ^C
      --- 192.168.2.5 ping statistics ---
      18 packets transmitted, 0 received, 100% packet loss, time 17406ms
      ```
      Are you sure you that you actually got an ip address assigned by the Jetson in [1. Checking IP](#1-checking-ip)?
