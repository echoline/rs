#!/bin/sh

FNAME="la_aventuroj_de_alicio_en_mirlando.txt"
LEN=4352
TAIL=`expr $LEN - 242`
HEAD=`expr $TAIL - 425`

if test `wc -l $FNAME | cut -f 1 -d ' '` -eq $LEN; then
  tail -n $TAIL $FNAME | head -n $HEAD | sed 's/$/ /g' | tr -d '\n' | tr -s ' '
fi

FNAME="alices_adventures_in_wonderland.txt"
LEN=3735
TAIL=`expr $LEN - 39`
HEAD=`expr $TAIL - 365`

if test `wc -l $FNAME | cut -f 1 -d ' '` -eq $LEN; then
  tail -n $TAIL $FNAME | head -n $HEAD | sed 's/$/ /g' | tr -d '\n' | tr -s ' '
fi
