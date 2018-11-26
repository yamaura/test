#!/bin/sh
echo -p Disk-Path(/dev/sda):
read disk
if [ ! -d $disk ]; then
  exit 1
fi
echo http://nl.alpinelinux.org/alpine/v3.8/main >> /etc/apk/repositories
apk update
apk add syslinux
