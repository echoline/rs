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

# Load sentence parser
#use Lingua::LinkParser;
#my $parser = new Lingua::LinkParser();

# Copyright (C) 1999 Lucent Technologies
# Excerpted from 'The Practice of Programming'
# by Brian W. Kernighan and Rob Pike

# markov.pl: markov chain algorithm for 2-word prefixes
my $NONWORD = "\n";
my %statetab;

sub addstates {
	my $w1 = $NONWORD;           # initial state
	my $w2 = $NONWORD;           # initial state
	$_ = join(' ', @_);
	print "markov chain: " . $_ . "\n";
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
	if ($w2 ne $NONWORD) {
		push(@{$statetab{$w1}{$w2}}, $NONWORD); 	# add tail
	}
	if (open(my $fh, '>', 'markov')) {
		print $fh Storable::freeze(\%statetab);
		close($fh);
	}
}

sub generate {
	my $w1;
	my $w2;
	my $suf = 0;
	my $ret = '';
	my @wcases = {};
	my @letters;

	$w1 = $w2 = $NONWORD;

#	if (int(rand(5)) > 1) {
#		my $choice = int(rand(scalar(@_) - 1));
#		for (my $i = 0; $i < $choice; $i++) {
#			pop;
#		}
#
#		while (!$suf) {
#			if (scalar(@_) > 1) {
#				$w2 = pop;
#				$w1 = pop;
#				push @_,$w1;
#			} elsif (scalar(@_) == 1) {
#				$w2 = pop;
#			}
#			$suf = $statetab{$w1}{$w2};
#			shift;
#		}
#	}

#	if ($w1 eq $NONWORD) {
#		if ($w2 eq $NONWORD) {
#			$ret = "";
#		} else {
#			$ret = $w2 . " ";
#		}
#	} else {
#		$ret = $w1 . " " . $w2 . " ";
#	}

	for (my $i = 0; $i < $#@; $i++) {
		@letters = $@[$i] =~ /./sg;
		
		my $case = AI::CBR::Case->new(
			word	=> {
				sim	=> \&sim_set,
			},
		);
		$case->set_values(
			word	=> [ @letters ],
		);
		push @wcases, $case;
	}

	for (my $i = 0; $i < 20; $i++) {
		$suf = $statetab{$w1}{$w2};	# array reference
		my $r = 0;
		for(my $j = 0; $j < 20; $j++) {
			$r = int(rand @$suf);		# @$suf is number of elems
			@letters = $suf->[$r] =~ /./sg;
			my $case = AI::CBR::Case->new(
				word	=> {
					sim	=> \&sim_set,
				},
			);
			$case->set_values(
				word	=> [ @letters ]
			);
			my $s = AI::CBR::Retrieval->new($case, \@wcases);
			$s->compute_sims();
			my $solution = $s->most_similar_candidate();
			if ($solution->{_sim} > 0.5) {
				print $solution->{_sim} * 100.0 . "% similarity";
				last;
			}
		}
		last if ((my $t = $suf->[$r]) eq $NONWORD);
		$ret = $ret . "$t ";
		($w1, $w2) = ($w2, $t);		# advance chain
	}

	$ret = $ret . " ";
	return $ret;
}

print "Initializing markov chain\n";
if (open(my $fh, '<', 'markov')) {
	my $stuff = '';
	while (<$fh>) {
		$stuff .= $_;
	}
	eval {
		%statetab = %{Storable::thaw($stuff)};
	};
	close($fh);
} else {
	addstates(":)");
}

#print "Initializing case-based reasoning\n";
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
$rs->loadDirectory ("/alice/replies");
print ".";
$rs->sortReplies;
print ". done.\n";

my $socket = "/tmp/alice";
unlink $socket;
print "Initializing socket\n";
my $server = IO::Socket::UNIX->new(Local => $socket,
				   Type => SOCK_STREAM,
				   Listen => SOMAXCONN) or die $@;
chmod(0600, $socket) || die $!;

print "Initialization complete.\n";

$SIG{'PIPE'} = sub { print "sigpipe\n"; };

$rs->{addcase} = sub {
	my $new_case = {
		isaid	=> $_[0],
		said	=> $_[1],
		words	=> [ $_[2] ],
	#	links	=> [ @links ],
	};
	push @cases, $new_case;
    	if (open(my $fh, '>', 'cases')) {
		print $fh Storable::freeze(\@cases);
		close($fh);
	}
};

# Start.
while (1) {
	my $client = $server->accept();
	my $buf;
	$client->recv($buf, 0x10000);
	my @inputstuff = split(/\007/, $buf);
	my $who = $inputstuff[0];

	if (!$who) {
#		addstates($inputstuff[1]);
		next;
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
	my @words;

	foreach (@msg_array) {
		if ($_ =~ /[A-Za-z0-9]/) {
#		if ($inputstuff[1] =~ /[A-Za-z0-9]/) {
			my $case;
			my $r;
			my $solution;

#			my $treply = $rs->reply($who, $inputstuff[1]);
			my $treply = $rs->reply($who, $_);
			if (length($treply) eq 0) {
				$treply = 'random pickup line';
			}
			my $said = $rs->{client}->{$who}->{__history__}->{input}->[0];
			@words = split(/\s+/, $said);

			#my $sentence = $parser->create_sentence($said);
			#if (!$sentence) {
			#	next;
			#}
			#my @bigstruct = $sentence->get_bigstruct;
			#my @links = [];

			#foreach(@bigstruct) {
			#	my $k;
			#	my $v;
			#	while (($k,$v) = each %{$_->{links}} ) {
			#		print " $k => " . $bigstruct[$v]->{word} . "\n";
			#		push (@links, $bigstruct[$v]->{word});
			#	}
			#}

			$case = AI::CBR::Case->new(
				said	=> {
					sim	=> \&sim_eq
				},
				words	=> {
					sim	=> \&sim_set
				},
			#	links	=> {
			#		sim	=> \&sim_set
			#	},
			);
			$case->set_values(
				said	=> $said,
				words	=> [ @words ],
			#	links	=> [ @links ],
			);

			$r = AI::CBR::Retrieval->new($case, \@cases);
			$r->compute_sims();
			$solution = $r->most_similar_candidate();
			print $solution->{_sim} * 100 . "% similarity\n";

			if ($treply !~ /random\ pickup\ line/) {
#				if ($solution->{_sim} < 0.5) {
				$rs->{addcase}->($treply, $said, @words);
#				}
				$reply .= $treply . ' ';
			} else {
				if ($solution->{_sim} < 0.9) {
					$treply = generate(@words);
					$reply .= $treply
				} else {
					$reply .= $solution->{isaid} . ' ';
					$treply = $solution->{isaid};
				}
			}
			$rs->{client}->{$who}->{__history__}->{reply}->[0] = $treply;
		}
	}
	if ($reply =~ /^\s*$/) {
		$reply = generate(@words);
#		$reply = ":)";
		$rs->{client}->{$who}->{__history__}->{reply}->[0] = $reply;
	} else {
		addstates($reply);
	}
	$rs->freezeUservars($who);
	if (exists($rs->{frozen}->{$who})
	    && open(my $fh, '>', 'sessions/' . $who)) {
		print $fh Storable::freeze( $rs->{frozen}->{$who} );
		close($fh);
	}
#	if ($reply =~ /^(who|what|where|why|when|how|is|are|were|was)/) {
#		print 'google: ' . $rs->reply('google', $reply) . "\n";
#	}

	$client->send($reply);
	print 'me: ' . $reply . "\n---" . time . "\n";
	$client->close;
}
