#!/bin/sh

while true; do read one; two=`./one $@ "$one"`; echo $two; done
