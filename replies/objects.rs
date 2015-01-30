+ xfind *
- <call>xfind <star></call>

> object xfind perl
	my ($self, @args) = @_;
	my $str = join("%20", @args);
	return "Try typing http://google.com/search?q=" . $str . "  ";
< object
