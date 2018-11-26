#!/bin/sh
echo -p Disk-Path(/dev/sda):
disk=/dev/sda

grep -q ${disk} /proc/partitions || exit 1
grep -q ${disk}1 /proc/partitions && exit 1

echo http://nl.alpinelinux.org/alpine/v3.8/main >> /etc/apk/repositories
apk update
apk add syslinux

