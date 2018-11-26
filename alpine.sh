#!/bin/sh
export disk=/dev/sda

grep -q ${disk} /proc/partitions || exit 1
grep -q ${disk}1 /proc/partitions && exit 2

cat | fdisk ${disk} <<EOF
n
p
1


a
1
t
c
w
EOF

modprobe vfat

echo http://nl.alpinelinux.org/alpine/v3.8/main >> /etc/apk/repositories
apk update
apk add syslinux

setup-bootable /media/cdrom ${disk}1
