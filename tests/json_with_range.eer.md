## input.json
```json
{"mamma": "MIA", "marcello": 45}
```

## grammar.inspi
```
<oggetto>: '{' <spazio> <chiavi_valore> <spazio> '}';
<chiavi_valore>: <chiave_valore> | <chiave_valore> <spazio> ',' <spazio> <chiavi_valore>;
<chiave_valore>: <stringa> <spazio> ':' <spazio> <valore>;

<valore>: <stringa> | <numero> | <booleano> | <null>;

<parola>: <lettera> | <lettera> <parola>;
<stringa>: '"' <parola> '"';
<spazio>: ' '+ | '';
<numero>: <cifra> | <cifra> <numero>;
<booleano>: 'true' | 'false';
<null>: 'null';

<lettera>: [A-Z] | [a-z] | '\"' | ' ' ;

<cifra>: [0-9];
```