#!/bin/bash

case "$(arch)" in
  aarch64) SUFFIX="aarch64";;
  armv6l) SUFFIX="armv6";;
esac

if [ -n "$SUFFIX" ]; then
  LEAFCAST_GZ="leafcast_linux_${SUFFIX}.tar.gz"
  wget "https://github.com/mrjackwills/leafcast_pi/releases/latest/download/${LEAFCAST_GZ}"
  tar xzvf "${LEAFCAST_GZ}" leafcast
  rm "${LEAFCAST_GZ}"
fi