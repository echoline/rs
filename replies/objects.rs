+ xfind *
- <call>xfind <star></call>

> object xfind perl
	use URI::Encode qw(uri_decode);
	use LWP::Simple qw (get);
	use JSON qw (decode_json);
	use Encode;
	use HTML::Strip;

	my ($self, @args) = @_;
	my $str = join("%20", @args);
	my $requrl = "https://ajax.googleapis.com/ajax/services/search/web?v=1.0&userip=50.7.6.178&q=" . $str;

	my $content = get $requrl;
	if (!$content) {
		return "broken fetch";
	}
	my $response = decode_json encode("ascii", $content);
	if (!$response) {
		return "malformed json";
	}

	my $hs = HTML::Strip->new();

	if ($response->{responseStatus} eq 200) {
		my $results = $response->{responseData}->{results};
		foreach (@$results) {
			if ($_->{url} =~ /^https?:\/\/(www.merriam-webster.com\/dictionary\/|en.wikipedia.org\/wiki\/|www.urbandictionary.com\/define.php).*/) {
				my $ret = $_->{content};
				$ret =~ s/\n//g;
				$ret = $hs->parse($ret);
				return $ret;
			}
		}
		foreach (@$results) {
			my $ret = uri_decode($_->{url});
			return $ret;
		}
	}
	return "http response code " . $response->{responseStatus};
< object

> object today perl
	use DateTime;
	my ($self, @args) = @_;

	return DateTime->now()->day_name;
< object

> object time perl
	return localtime();
< object

> object weather perl
	use Weather::Underground;

	my ($self, @args) = @_;
	my $loc = join(' ', @args);
	my $ret = '';

	my $weather = Weather::Underground->new(place => $loc);
	if (!$weather) {
		return 'invalid location';
	}

	my $info = $weather->get_weather();
	if (!$info) {
		return 'invalid location';
	}

	foreach (@$info) {
		while ((my $key, my $value) = each %{$_}) {
			if ($key =~ /(conditions|wind_milesperhour|humidity|wind_direction|temperature_fahrenheit)/) {
				$ret = $ret . $key . ": " . $value . ", ";
			}
		}
	}

	$ret = substr $ret, 0, -2;

	return $ret;
< object
