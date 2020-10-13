# Koda
Koda is a small application that provides an "abstraction layer" on top of the Lua runtime with Swedish keywords, variable names and functions.

### Why create such a thing?
Well, mostly just for fun and as a programming exercise. But a language/application such as Koda might be a bit useful if you try explaining programming to young children. Syntax in English can become barrier for young non-English speakers when trying to learn fundamental programming concepts. And being able to show a working code snippet with syntax in their native language can possibly make it easier to understand, in the same way as visual programming can be a helpful educational tool.

However, it is of course preferable to learn an actually usable language with conventional syntax. 

### How do I use Koda?
[Download the executable file](https://github.com/adelhult/koda/releases/tag/v0.1), and drag a *.kod* or *.lua* file on top of it to run that file. Alternatively, you can of course add Koda to your path and run:
`koda filename.kod`

### Todo
* Update manual/readme
* create new binaries
* somewhat stable (and nicer) api, more global functions should be added and perhaps adjust a few keywords
* Add the missing tokens (multi-line comment and "...")

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
om namn == "anna" utför
    skriv("Hej Anna!")
annarsom namn == "erik" utför
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
slumpa(1, 100)

-- slumpa ett tal mellan 1 - 6
tärning()

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

-- Fler funktioner:
-- gemener(s)       - kallar på string.lower(s)
-- versaler(s)      - kallar på string.upper(s)
-- tillsträng(n)    - kallar på tostring(n)
-- tillnummer(s)    - kallar på tonumber(s)

-- Alla nyckelord:
-- och       - and
-- bryt      - break
-- gör       - do
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
-- utför     - then
-- medan     - while
```

### Vim installation
To install syntax highlighting for koda, put the [vim file](koda.vim) in `~/.vim/syntax/` and add the following line to `~/.vim/ftdetect/kod.vim`. Where `~/.vim/` is your default vim directory.

```vim
au BufRead,BufNewFile *.kod set filetype=koda
```
A nice example of [koda in Gruvbox](https://imgur.com/a/rZOq8yZ).
