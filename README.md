# replacer

Replaces the variables listed in a json variables file in to the template file and saves the output.

## variables json file layout
```json
{
    "vars": {
        "%env%": "D",
        "THIS": "<this>"
    }
}
```

##cli arguments
```txt
usage: replacer <source> <variables> <encode: html|txt> [dest]
```

source: is the template file containing the tokes you wish to replace.
variables: is the json file containing the key value pair, the key is looked for in the template and when found replaced with the value.
encode: html for encoding the value into html and txt (or any other value) for none encoding
dest: is the optional output file where the replaced data is saved. When ommited the source file is used as the output file

## template example
```txt
This is a template for %env%
Where you see %env% it should say D
Where we see THIS it should say this.
```

## encoding values into html
You can encode the values from the variables json file into different encodings, like html
```txt
replacer template.txt vars.json html out1.txt
```

## encoding values into txt (no encoding)
You can encode the values from the variables json file into different encodings, like html
```txt
replacer template.txt vars.json txt out1.txt
```
