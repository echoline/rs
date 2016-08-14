#!/bin/sh
# requires superuser rights

apt-get install liblink-grammar4
cpan -i "AI::CBR"
cpan -i "RiveScript"
cpan -i "Lingua::LinkParser"
