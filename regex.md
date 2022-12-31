# REGEX


### BASICS 

```
^Adam => Begin by
Adam$ => End by
^Adam$ => String Match exactly by
Adam|Hanna => Or
(Adam|Hanna)boo => Match with Hannaboo or Adamboo
e+ => match e or multiple e
e? => match with e but e can be optionnal
e* => combine + and ? (Multiple char and optionnal char)
.et => match with all occurence (tet vet 3et...) (Except a new line)
```

### LISTS
```
[tl]itre => Match with titre or litre
[a-z] => Intervall
```

### RANGE
```
\w{5} => Match five charachter exactly
\w{5,} => Match Five or more characters
\w{5,10} => Match Five to Ten characher
```

### SPECIAL
```
\w => Match with any character
\W => Match with anithing is not a character
\s => Match with any whitespace
\S => Match with anything is not a space
```
