## input.txt
```
mi toki la jan Loleniso li lape
```

## grammar.inspi
```
<sentence>: <adverb> <space> 'la' <space> <simple_sentence> | <simple_sentence>;
<adverb>: <noun_phrase> | <simple_sentence>;
<subject>: <noun_phrase>;
<simple_sentence>: <subject> <space> 'li' <space> <predicate>;
<noun_phrase>: <word> | <word> <space_and_word>;
<word>: 'mi' | 'sina' | 'ona' | 'pona' | 'toki' | 'nimi' | 'li'
    | 'mani' | 'lape' | 'tomo' | 'tawa' | 'jan' | 'ma' | <loan_word>;
<loan_word>: [A-Z] [a-z]+;
<space_and_word>: <space>+ <word>;
<space>: ' '+;
<predicate>: <noun_phrase>;
```