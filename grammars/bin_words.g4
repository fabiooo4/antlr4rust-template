grammar bin_words;

main: bin EOF;
bin: | '0' bin | '1' bin;
