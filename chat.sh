#!/bin/sh

while true; do echo -n \>\ ; read one; two=`./one "$one"`; echo $two; espeak -s 130 -v f3 "$two" --stdout | aplay; done
