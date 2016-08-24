#!/bin/sh

FNAME="la_aventuroj_de_alicio_en_mirlando.txt"

LEN=`wc -l $FNAME | cut -f 1 -d ' '`
TAIL=`expr $LEN - 242`
HEAD=`expr $TAIL - 425`

tail -n $TAIL $FNAME | head -n $HEAD | tr -s ' ' | tr -d '\n'
