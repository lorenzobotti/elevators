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
<spazio>: ' ' | '';
<numero>: <cifra> | <cifra> <numero>;
<booleano>: 'true' | 'false';
<null>: 'null';

<lettera>: 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G'
       | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N'
       | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U'
       | 'V' | 'W' | 'X' | 'Y' | 'Z' | 'a' | 'b'
       | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i'
       | 'j' | 'k' | 'l' | 'm' | 'n' | 'o' | 'p'
       | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w'
       | 'x' | 'y' | 'z' | '\"' | ' ' ;

<cifra>: 
   '0' | '1' | '2'
 | '3' | '4' | '5'
 | '6' | '7' | '8'
 | '9';
```