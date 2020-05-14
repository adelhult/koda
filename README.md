# Koda
Koda is a small application that provides an "abstraction layer" on top of the Lua runtime with Swedish keywords, variable names and functions.

### Why create such a thing?
Well, mostly just for fun and as a programming exercise. But a language/application such as Koda might be a bit useful if you try explaining programming to younger children. Syntax in English can become barrier for young non-English speakers when trying to learn fundamental programming concepts. And being able to show a working code snippet with syntax in their native language can possibly make it easier to understand, in the same way as visual programming can be a helpful educational tool.

However, it is of course preferable to learn an actually usable language with conventional syntax.

### How do use Koda?
Download the executable file, and drag a *.kod* or *.lua* file on top of it to run that file. Alternatively, you can of course add Koda to your path and run:
`koda filename.kod`

### Example code
```
-- Koda, exempelkod
-- (Notera att all lua-kod också är giltig och går att skriva
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
medans x > 100 gör
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

-- hämta namnet på de fil som körs:
skriv(_filnamn)

-- hämta eventuella cli argument:
för k, v i ipar(_parametrar) gör
    skriv(v)
slut

-- Fler funktioner:
-- gemener(s)       - kallar på string.lower(s)
-- versaler(s)       - kallar på string.upper(s)
-- tillsträng(n)    - kallar på tostring(n)
-- tillnummer(s)    - kallar på tonumber(s)

-- Alla nyckelord:
-- och      - and
-- bryt     - break
-- do       - gör
-- annars   - else
-- annarsom - elseif
-- slut     - end
-- falskt   - false
-- sant     - true
-- för      - for
-- funktion - function
-- om       - if
-- i        - in
-- lokal    - local
-- nil      - nil
-- inte     - not
-- eller    - or
-- upprepa  - repeat
-- tills    - until
-- ge       - return
-- utför    - then
-- medans   - while
```
