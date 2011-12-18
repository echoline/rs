> object weather perl
use IO::Socket::INET;

my ($self, @l) = @_;

my $loc = join(' ', @l);
my $locenc = $loc;
$locenc =~ s/(.)/sprintf("%%%x",ord($1))/eg;

my $sock = IO::Socket::INET->new('www.google.com:80');
$sock->autoflush(1);

print $sock "GET /ig/api?weather=" . $locenc . " HTTP/1.0  ";

my $r = '';
while (<$sock>) {
        $r .= $_;
}

close $sock;

if ($r =~ /problem/i) {
	return "Problem fetching weather data for " . $loc . ".  ";
}

$r =~ s/^.*\r \r (.*)$/$1/smg;
$r =~ s/^.*\<current_conditions\>(.*)\<\/current_conditions\>.*$/$1/;

$r =~ s/\<condition data=\"([^\"]*)\"\/?\>/Conditions are $1.  /;
$r =~ s/\<temp_f data=\"([^\"]*)\"\/?\>/$1 degrees Fahrenheit.  /;
$r =~ s/\<temp_c data=\"([^\"]*)\"\/?\>/$1 degrees Celsius.  /;
$r =~ s/\<humidity data=\"([^\"]*)\"\/?\>/$1.  /;
$r =~ s/\<icon data=\"([^\"]*)\"\/?\>//;
$r =~ s/\<wind_condition data=\"([^\"]*)\"\/?\>/$1.  /;

return $r;
< object

> object stocks perl
use IO::Socket::INET;

my ($self, @l) = @_;

my $loc = join(' ', @l);
my $locenc = $loc;
$locenc =~ s/(.)/sprintf("%%%x",ord($1))/eg;

my $sock = IO::Socket::INET->new('www.google.com:80');
$sock->autoflush(1);

print $sock "GET /ig/api?stock=" . $locenc . " HTTP/1.0  ";

my $r = '';
while (<$sock>) {
        $r .= $_;
}

close $sock;

if ($r =~ /UNKNOWN EXCHANGE/) {
        return $loc . " is an unknown exchange symbol.  ";
}

if ($r !~ /company/) {
        return "Probable error between keyboard and chair.  ";
}

$r =~ s/^.*\r \r (.*)$/$1/smg;

my $inf;

my $currency = $r;
$currency =~ s/^.*\<currency data=\"([^\"]+)\"\/?\>.*$/$1/;

my $last = $r;
$last =~ s/^.*\<last data=\"([^\"]+)\"\/?\>.*$/$1/;

my $change = $r;
$change =~ s/^.*\<change data=\"([^\"]+)\"\/?\>.*$/$1/;

if ($change =~ /^\+/) {
        $inf = $loc . " is up " . $change . " at a price of " . $last . " " . $currency . ".  ";
} else {
        $inf = $loc . " is down " . $change . " at a price of " . $last . " " . $currency . ".  ";
}

return $inf;
< object

> object movies perl
use IO::Socket::INET;

my ($self, @l) = @_;

my $loc = join(' ', @l);
my $locenc = $loc;
$locenc =~ s/(.)/sprintf("%%%x",ord($1))/eg;

my $sock = IO::Socket::INET->new('www.google.com:80');
$sock->autoflush(1);

print $sock "GET /ig/api?movies=" . $locenc . " HTTP/1.0  ";

my $r = '';
while (<$sock>) {
        $r .= $_;
}

close $sock;

$r =~ s/^.*\r \r (.*)$/$1/smg;

if ($r =~ /problem_cause/) {
	my $prob = $r;
	$prob =~ s/^.*\<problem_cause data=\"([^\"]+)\"\/?\>.*$/$1/;
	return $prob;
}

my $movies = "Movies playing near " . $loc . ": ";
my @list = ($r =~ /\<title data=\"([^\"]+)\"\/?\>/smg);
$movies .= join(", ", @list) . ".  ";

return $movies;
< object

+ xfind *
- <call>xfind <star></call>

> object xfind perl
	my ($self, @args) = @_;
	my $str = join("%20", @args);
	return "http://lmgtfy.com/?q=" . $str;
< object
