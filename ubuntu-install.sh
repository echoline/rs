#!/bin/sh
# requires superuser rights

sudo apt-get install liblink-grammar4-dev
cpan -i "AI::CBR"
cpan -i "RiveScript"
cpan -i "Lingua::LinkParser"
