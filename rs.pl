#!/usr/bin/perl -w

use strict;
use warnings;
use IO::Socket;
use Storable;

# Use our local library.
# use lib ".";

# Case-based reasoning
use AI::CBR::Sim qw(sim_set sim_eq);
use AI::CBR::Case;
use AI::CBR::Retrieval;

# Sentence parser
use Lingua::LinkParser;

# Load RiveScript.
use RiveScript;

print "Initializing sentence parser and case-based reasoning\n";
our $parser = new Lingua::LinkParser;
our @cases = {};

# Create and load the RiveScript brain.
print "Initializing RiveScript interpreter\n";
our $rs = new RiveScript;
$rs->loadDirectory ("./replies");
$rs->sortReplies;

my $socket = "/tmp/rs";
unlink $socket;
print "Initializing socket\n";
my $server = IO::Socket::UNIX->new(Local => $socket,
				   Type => SOCK_STREAM,
				   Listen => SOMAXCONN) or die $@;
chmod(0777, $socket) || die $!;

print "Initialization complete.\n";

# Start.
while (1) {
	my $client = $server->accept();
	my $buf;
	$client->recv($buf, 1024);
	my ($who, $msg) = split(/\007/, $buf);
	print $who . ': ' . $msg . "\n";
	my $reply = '';
	my @msg_array = split(/[\.\?\!]/, $msg);

	if (open(my $fh, '<', 'sessions/' . $who)) {
		my $stuff = '';
		while (<$fh>) {
			$stuff .= $_;
		}
		$rs->{frozen}->{$who} = Storable::thaw($stuff);
		close($fh);
	}

	$rs->thawUservars($who);
	foreach (@msg_array) {
		if ($_ =~ /[a-zA-Z0-9]/) {
			my $treply = $rs->reply($who, $_);
			if (length($treply) eq 0) {
				$treply = 'random pickup line';
			}
			my $said = $rs->{client}->{$who}->{__history__}->{input}->[0];
			my $sentence = $parser->create_sentence($said);
			if (!$sentence) {
				next;
			}
			my @bigstruct = $sentence->get_bigstruct;
			my @links = [];

			foreach(@bigstruct) {
				my $k;
				my $v;
				while (($k,$v) = each %{$_->{links}} ) {
					push (@links, $k . $bigstruct[$v]->{word});
				}
			}

			my $case = AI::CBR::Case->new(
				said	=> {
					sim	=> \&sim_eq
				},
				words	=> {
					sim	=> \&sim_set
				},
				links	=> {
					sim	=> \&sim_set
				},
			);
			$case->set_values(
				said	=> $said,
				words	=> [ split(/\s+/, $said) ],
				links	=> @links,
			);

			my $r = AI::CBR::Retrieval->new($case, \@cases);
			$r->compute_sims();
			my $solution = $r->most_similar_candidate();
			print $solution->{_sim} * 100 . "% similarity\n";

			if ($treply !~ /random\ pickup\ line/) {
				if ($solution->{_sim} ne 1) {
					my $new_case = {
						isaid	=> $treply,
						said	=> $said,
						words	=> [ split(/\s+/, $said) ],
						links	=> @links,
					};
					push @cases, $new_case;
				}
				$reply .= $treply . '  ';
			} else {
				if ($solution->{_sim} ne 0) {
					$reply .= $solution->{isaid} . '  ';
				} else {
					$reply .= $rs->reply($who, 'random pickup line') . '  ';
				}
			}
		}
	}
	if ($reply =~ /^$/) {
		$reply = ":)";
	}
	$rs->freezeUservars($who);
	if (open(my $fh, '>', 'sessions/' . $who)) {
		print $fh Storable::freeze( $rs->{frozen}->{$who} );
		close($fh);
	}
	
	$client->send($reply);
	print 'me: ' . $reply . "\n---" . time . "\n";
	$client->close;
}
