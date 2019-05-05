#!/usr/bin/perl -w

my $f0 = $ARGV[0];

open(ONE, '<'.$f0);
while(<ONE>) {
	if (/^\+\ /) {
		print $_;
		$input = quotemeta($_);
		for ($i = 1; $i < scalar(@ARGV); $i++) {
			open(TWO, '<'.$ARGV[$i]);
			while(<TWO>) {
				if (/^$input$/) {
					while(<TWO>) {
						if (/^$/) {
							last;
						}
						print $_;
					}
				}
			}
		}
	} else {
		print $_;
	}
}
