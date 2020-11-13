pub fn get_prelude() -> &'static str {
    r#"
      -- Koda translations for a small and simple
      -- selection of the Lua standard library functions
      -- visit: www.lua.org/manual/5.3/manual.html
       
      s__ae__kerst__ae__ll = assert
      fel = error
      h__ae__mtametatabell = getmetatable
      ipar = ipairs 
      ladda = load
      laddafil = loadfile
      n__ae__sta = next
      par = pairs
      v__ae__lj = select
      angemetatabell = setmetatable
      tillnummer = tonumber
      tillstr__ae__ng = tostring
      typ = type -- borde det vara på svenska? Jag tycker ja
      beh__oe__ver = require -- kan bara importera lua kod, måste gör en rust funktion för att ladda koda-kod
      str__ae__ng = {}
      str__ae__ng.bit = string.byte
      str__ae__ng.tecken = string.char
      str__ae__ng.dumpa = string.dump
      str__ae__ng.hitta = string.find
      str__ae__ng.formatera = string.format

        gemener = string.lower
        versaler = string.upper
        slumpa = math.random
        function t__ae__rning() return math.random(6) end
    "#
}

/*
nyttja metatables och skriv om dem!
mt = getmetatable("")

for _, functions in pairs(mt) do
  for k,v in pairs(functions) do
    print(k,v)
  end
end

*/