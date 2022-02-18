# replacer

Replaces the variables listed in a json variables file in to the template file and saves the output.

## vars.json file layout
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
