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

## template example
```txt
This is a template for %env%
Where you see %env% it should say D
Where we see THIS it should say this.
```

## encoding values into html
You can encode the values from the variables json file into different encodings, like html
```txt
target/release/replacer template.txt vars.json html out1.txt
```

## encoding values into txt (no encoding)
You can encode the values from the variables json file into different encodings, like html
```txt
replacer template.txt vars.json txt out1.txt
```
