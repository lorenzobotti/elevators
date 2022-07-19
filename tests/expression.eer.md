## input
```
1+2*3+(4)
```

## grammar.inspi
```
<fattore>: <somma> | <somma> '*' <atomo>; 
<somma>: <atomo> | <atomo> '+' <fattore>;
<atomo>: <numero> | '(' <fattore> ')';
<numero>: 
   '0' | '1' | '2'
 | '3' | '4' | '5'
 | '6' | '7' | '8'
 | '9';
```