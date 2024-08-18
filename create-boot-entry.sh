#!/bin/sh

params="root=UUID=a205c217-c83a-4b74-b77d-3a8ec41330c4 rootfstype=ext4 rw \
  cryptroot=UUID=641a2ab6-faf0-4f76-a307-fd3885b45a8c cryptdm=root \
  initrd=\intel-ucode.img \ 
  initrd=\initramfs-lts"

efibootmgr --create --label "Alpine Linux - lts" \
  --disk /dev/sda --part 1 \
  --loader /vmlinuz-lts \
  --unicode "${params}" \
  --verbose
