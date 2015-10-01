#!/usr/bin/perl -w

use strict;
use warnings;
use IO::Socket;
use Storable;

# Case-based reasoning
use AI::CBR::Sim qw(sim_set sim_eq);
use AI::CBR::Case;
use AI::CBR::Retrieval;

# Load RiveScript.
use RiveScript;

print "Initializing markov chain\n";
my %statetab;
if (open(my $fh, '<', 'markov')) {
	my $stuff = '';
	while (<$fh>) {
		$stuff .= $_;
	}
	eval {
		%statetab = %{Storable::thaw($stuff)};
	};
	close($fh);
}

# Copyright (C) 1999 Lucent Technologies
# Excerpted from 'The Practice of Programming'
# by Brian W. Kernighan and Rob Pike

# markov.pl: markov chain algorithm for 2-word prefixes

my $NONWORD = "\n";
my $w1 = $NONWORD;           # initial state
my $w2 = $NONWORD;           # initial state

sub addstates {
	foreach (split) {
		my $t = $_;
		$t =~ s/\"//g;
		push(@{$statetab{$w1}{$w2}}, $t);
		($w1, $w2) = ($w2, $t);	# multiple assignment
		if ($t =~ /[\.\?\!]$/) {
			push(@{$statetab{$w1}{$w2}}, $NONWORD); 	# add tail
			$w1 = $w2 = $NONWORD;
		}
	}
	push(@{$statetab{$w1}{$w2}}, $NONWORD); 	# add tail
	if (open(my $fh, '>', 'markov')) {
		print $fh Storable::freeze( %statetab );
		close($fh);
	}
}

sub generate {
	my $ret = "";
	$w1 = $w2 = $NONWORD;
	for (my $i = 0; $i < 10000; $i++) {
		my $suf = $statetab{$w1}{$w2};	# array reference
		my $r = int(rand @$suf);		# @$suf is number of elems
		last if ((my $t = $suf->[$r]) eq $NONWORD);
		$ret = $ret . "$t ";
		($w1, $w2) = ($w2, $t);		# advance chain
	}

	$ret = $ret . " ";
	return $ret;
}

print "Initializing case-based reasoning\n";
my @cases = {};
if (open(my $fh, '<', 'cases')) {
	my $stuff = '';
	while (<$fh>) {
		$stuff .= $_;
	}
	eval {
		@cases = @{Storable::thaw($stuff)};
	};
	close($fh);
}

# Create and load the RiveScript brain.
print "Initializing RiveScript interpreter ";
our $rs = new RiveScript;
print ".";
$rs->loadDirectory ("./replies");
print ".";
$rs->sortReplies;
print ". done.\n";

my $socket = "/tmp/alice";
unlink $socket;
print "Initializing socket\n";
my $server = IO::Socket::UNIX->new(Local => $socket,
				   Type => SOCK_STREAM,
				   Listen => SOMAXCONN) or die $@;
chmod(0777, $socket) || die $!;

print "Initialization complete.\n";

$SIG{'PIPE'} = sub { print "sigpipe\n"; };

# Start.
while (1) {
	my $client = $server->accept();
	my $buf;
	$client->recv($buf, 0x10000);
	my @inputstuff = split(/\007/, $buf);
	my $who = $inputstuff[0];

	if ($who =~ /^$/) {
		addstates($inputstuff[1]);
	}

	if (open(my $fh, '<', 'sessions/' . $who)) {
		my $stuff = '';
		while (<$fh>) {
			$stuff .= $_;
		}
		eval {
			$rs->{frozen}->{$who} = Storable::thaw($stuff);
		};
		close($fh);
	}

	if (defined($rs->{frozen}->{$who})) {
		$rs->thawUservars($who);
	}

	if (scalar(@inputstuff) eq 1) {
		if (exists($rs->{client}->{$who}) && exists($rs->{client}->{$who}->{personality})) {
			$client->send($rs->{client}->{$who}->{personality});
		} else {
			$client->send('undefined');
		}
		
		$client->close;
		next;
	}

	if (scalar(@inputstuff) ne 2) {
		$client->send('arity of ' . scalar(@inputstuff) . '?');
		$client->close;
		next;
	}

	print $who . ': ' . $inputstuff[1] . "\n";
	my $reply = '';
	chomp($inputstuff[1]);
	my @msg_array = split(/[\.\!\?](\s+|$)/, $inputstuff[1]);

	foreach (@msg_array) {
		if ($_ =~ /[a-zA-Z0-9]/) {
			my $case;
			my $r;
			my $solution;

#			if (exists($rs->{client}->{$who}->{__history__}) && exists($rs->{client}->{$who}->{__history__}->{reply}->[1])) {
#				my $initiation = $rs->{client}->{$who}->{__history__}->{reply}->[1];
#				my $response = $rs->{client}->{$who}->{__history__}->{input}->[0];
#			
#				$sentence = $parser->create_sentence($initiation);
#				if ($sentence) {
#					@bigstruct = $sentence->get_bigstruct;
#					@links = [];
#
#					foreach(@bigstruct) {
#						my $k;
#						my $v;
#						while (($k,$v) = each %{$_->{links}} ) {
#							push (@links, $k . $bigstruct[$v]->{word});
#						}
#					}
#
#					$case = AI::CBR::Case->new(
#						said	=> {
#							sim	=> \&sim_eq
#						},
#						words	=> {
#							sim	=> \&sim_set
#						},
#						links	=> {
#							sim	=> \&sim_set
#						},
#					);
#					$case->set_values(
#						said	=> $initiation,
#						words	=> [ split(/\s+/, $initiation) ],
#						links	=> @links,
#					);
#
#					$r = AI::CBR::Retrieval->new($case, \@cases);
#					$r->compute_sims();
#					$solution = $r->most_similar_candidate();
#					if ($solution->{_sim} ne 1) {
#						my $new_case = {
#							isaid	=> $response,
#							said	=> $initiation,
#							words	=> [ split(/\s+/, $initiation) ],
#							links	=> @links,
#						};
#						push @cases, $new_case;
#					}
#				}
#			}

			my $treply = $rs->reply($who, $_);
			if (length($treply) eq 0) {
				$treply = 'random pickup line';
			}
			my $said = $rs->{client}->{$who}->{__history__}->{input}->[0];
#			$sentence = $parser->create_sentence($said);
#			if (!$sentence) {
#				next;
#			}
#			@bigstruct = $sentence->get_bigstruct;
#			@links = [];

#			foreach(@bigstruct) {
#				my $k;
#				my $v;
#				while (($k,$v) = each %{$_->{links}} ) {
#					push (@links, $k . $bigstruct[$v]->{word});
#				}
#			}

			$case = AI::CBR::Case->new(
				said	=> {
					sim	=> \&sim_eq
				},
				words	=> {
					sim	=> \&sim_set
				},
#				links	=> {
#					sim	=> \&sim_set
#				},
			);
			$case->set_values(
				said	=> $said,
				words	=> [ split(/\s+/, $said) ],
#				links	=> @links,
			);

			$r = AI::CBR::Retrieval->new($case, \@cases);
			$r->compute_sims();
			$solution = $r->most_similar_candidate();
			print $solution->{_sim} * 100 . "% similarity\n";

			if ($treply !~ /random\ pickup\ line/) {
				if ($solution->{_sim} ne 1) {
					my $new_case = {
						isaid	=> $treply,
						said	=> $said,
						words	=> [ split(/\s+/, $said) ],
#						links	=> @links,
					};
					push @cases, $new_case;
	    				if (open(my $fh, '>', 'cases')) {
						print $fh Storable::freeze(\@cases);
						close($fh);
					}
				}
				$reply .= $treply . '  ';
			} else {
				if ($solution->{_sim} gt 0.5) {
					$reply .= $solution->{isaid} . '  ';
				} else {
					$reply .= generate();
				}
			}
		}
	}
	if ($reply =~ /^$/) {
		$reply = ":)";
	}
	$rs->freezeUservars($who);
	if (exists($rs->{frozen}->{$who})
	    && open(my $fh, '>', 'sessions/' . $who)) {
		print $fh Storable::freeze( $rs->{frozen}->{$who} );
		close($fh);
	}
	
	$client->send($reply);
	print 'me: ' . $reply . "\n---" . time . "\n";
	$client->close;
}
