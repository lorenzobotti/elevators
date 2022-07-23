## input.txt
```
nananananananananananananananana batman!!!!!
```

## grammar.inspi
```
<sound>: <jingle> <space> <exclamation>;
<jingle>: 'na'*;
<space>: ' '+ | '';
<exclamation>: 'batman' | 'batman' <exclamation_marks>;
<exclamation_marks>: '!'+;
```