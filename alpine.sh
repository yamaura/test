#!/bin/sh
export disk=/dev/sda

grep -q ${disk##*/} /proc/partitions || exit 1
grep -q ${disk##*/}1 /proc/partitions && exit 2

modprobe vfat

echo http://nl.alpinelinux.org/alpine/v3.8/main >> /etc/apk/repositories
apk update
apk add syslinux dosfstools


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

mkdosfs -F32 ${disk}1
setup-bootable /media/cdrom ${disk}1
sync
