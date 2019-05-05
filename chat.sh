#!/bin/sh

while true; do echo -n \>\ ; read one; two=`./one $@ "$one"`; echo $two; done
