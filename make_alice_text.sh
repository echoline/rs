#!/bin/sh

FNAME="alices_adventures_in_wonderland.txt"
LEN=3735
TAIL=`expr $LEN - 39`
HEAD=`expr $TAIL - 365`

if test `wc -l $FNAME | cut -f 1 -d ' '` -eq $LEN; then
  tail -n $TAIL $FNAME | head -n $HEAD | sed 's/$/ /g' | tr -d '\n' | tr -s ' '\
	| sed "s/[\"_]//g"
fi

FNAME="la_aventuroj_de_alicio_en_mirlando.txt"
LEN=4352
TAIL=`expr $LEN - 242`
HEAD=`expr $TAIL - 425`

if test `wc -l $FNAME | cut -f 1 -d ' '` -eq $LEN; then
  tail -n $TAIL $FNAME | head -n $HEAD | sed 's/$/ /g' | tr -d '\n' | tr -s ' '\
	| sed "s/[\"_]//g" | sed -e "s/\xC4\x88/Cx/g" -e "s/\xc4\x89/cx/g" \
	-e "s/\xc4\x9c/Gx/g" -e "s/\xc4\x9d/gx/g" -e "s/\xc4\xa4/Hx/g" \
	-e "s/\xc4\xa5/hx/g" -e "s/\xc4\xb4/Jx/g" -e "s/\xc4\xb5/jx/g" \
	-e "s/\xc5\x9c/Sx/g" -e "s/\xc5\x9d/sx/g" -e "s/\xc5\xac/Ux/g" \
	-e "s/\xc5\xad/ux/g"
fi
