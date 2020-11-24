<img src="https://raw.githubusercontent.com/adelhult/koda/master/icon.png" width="250px" align="left">

**Koda is a programming language with both keywords and variable names in Swedish. The Koda software transpiles your code to Lua which is then directly executed in a Lua runtime.** Basically, you can think of Koda as a small abstraction of top of Lua that just replaces the name of the keywords. Most of the grammar is the same as Lua’s, you can even use the Lua standard library in your code. However, Koda also provides a prelude of function translated into Swedish.
### Why create such a thing?
Well, mostly just for fun and as a programming exercise. But a language/application such as Koda might be a bit useful if you try explaining programming to young children. Syntax in English can become barrier for young non-English speakers when trying to learn fundamental programming concepts. And being able to show a working code snippet with syntax in their native language can possibly make it easier to understand, in the same way as visual programming can be a helpful educational tool. However, it is of course preferable to learn an actually usable language with conventional syntax. 

### How do I use Koda?
[Download the executable file](https://github.com/adelhult/koda/releases/tag/v0.2), and drag a *.kod* file on top of it to run that file. Alternatively, you can of course run it from your command-line:
`koda filename.kod`
Koda also comes with a REPL, start it by execting the program without giving any arguments.

### Example code
```
-- Koda, exempelkod
-- (Notera att alla lua-funktionsutrop är giltiga och går att använda
-- precis som vanligt.)

-- skriv text till användaren
skriv("Hej på dig!")

-- hämta en sträng från användaren
namn = fråga("Vad är ditt namn?")

-- if-satser
om namn == "anna" gör
    skriv("Hej Anna!")
annarsom namn == "erik" gör
    skriv("Hej Erik!")
annars
    skriv("Hej " .. namn .. "! Kul att träffa dig.")
slut

-- tabeller
telefonbok = {}
telefonbok.eli = "0701234456"
telefonbok.anna = "0734563245"

-- for-loop
-- använd ipar() eller par().
-- samma som ipairs och pairs
för person, nr i par(telefonbok) gör
    skriv(person)
    skriv(nr)
slut

-- while-loop
x = 0
medan x > 100 gör
    skriv("hej!")
slut

-- repeat...until
upprepa
    skriv("hej!")
tills (x < 10)

-- slumpa fram ett tal mellan
-- 1 och 100. (Kallar på math.random(a, b))
matte.slumpa(1, 100)

-- öppna en katalog i en filutforskare
-- eller en url i en webbläsare
öppna("www.google.com")

-- funktioner
funktion plus_tio(x)
    ge (x + 10)
slut

-- hämta namnet på den fil som körs:
skriv(_FILNAMN)

-- hämta eventuella cli argument:
för k, v i ipar(_PARAMETRAR) gör
    skriv(v)
slut

-- Alla nyckelord:
-- och       - and
-- bryt      - break
-- gör       - do || then (beroende på kontexten)
-- annars    - else
-- annarsom  - elseif
-- slut      - end
-- falskt    - false
-- sant      - true
-- för       - for
-- funktion  - function
-- om        - if
-- i         - in
-- lokal     - local
-- ingenting - nil
-- inte      - not
-- eller     - or
-- upprepa   - repeat
-- tills     - until
-- ge        - return
-- medan     - while
```

### Vim installation
To install syntax highlighting for koda, put the [vim file](koda.vim) in `~/.vim/syntax/` and add the following line to `~/.vim/ftdetect/kod.vim`. Where `~/.vim/` is your default vim directory.

```vim
au BufRead,BufNewFile *.kod set filetype=koda
```
A nice example of [koda in Gruvbox](https://imgur.com/a/rZOq8yZ).
