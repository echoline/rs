#!/usr/bin/perl
# Copyright (C) 1999 Lucent Technologies
# Excerpted from 'The Practice of Programming'
# by Brian W. Kernighan and Rob Pike

# markov.pl: markov chain algorithm for 2-word prefixes
use Storable;

$NONWORD = "\n";
$w1 = $w2 = $NONWORD;           # initial state

if (open($fh, '<', 'markov')) {
	$stuff = '';
	while (<$fh>) {
		$stuff .= $_;
	}
	eval {
		%statetab = %{Storable::thaw($stuff)};
	};
	close($fh);
}

while (<>) {                    # read each line of input
	foreach (split) {
		$t = $_;
		$t =~ s/\"//g;
		push(@{$statetab{$w1}{$w2}}, $t);
		($w1, $w2) = ($w2, $t);	# multiple assignment
		if ($t =~ /[\.\?\!]$/) {
			push(@{$statetab{$w1}{$w2}}, $NONWORD); 	# add tail
			$w1 = $w2 = $NONWORD;
		}
	}
}

if (open($fh, '>', 'markov')) {
	print $fh Storable::freeze(\%statetab);
	close($fh);
}
