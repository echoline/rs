#!/bin/sh

while read line; do
	echo "$line" > /dev/shm/line
	if ! grep -qE '^[0-2][0-9]:[0-5][0-9]:[0-5][0-9]\ [^\ ]+\ → (echolina|alice)[,:]' /dev/shm/line; then continue; fi
	name=`echo "$line" | sed -e "s,^[0-2][0-9]:[0-5][0-9]:[0-5][0-9]\ ,," | sed -e "s, .*$,,"`;
	msg=`echo "$line" | sed -Ee "s,^[0-2][0-9]:[0-5][0-9]:[0-5][0-9]\ [^\ ]+\ [^\ ]+\ (echolina|alice)[\,:] ,,"`;
	~/Documents/rs/one -u "$name" "$msg" | tr -s " "
done
