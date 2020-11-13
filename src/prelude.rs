pub fn get_prelude() -> &'static str {
    r#"
      -- Swedish translations for a subset of the 
      -- Lua standard library functions, to read more about them
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
      ange_metatabell = setmetatable
      till_nummer = tonumber
      till_str__ae__ng = tostring
      typ = type 
      str__ae__ng = {}
      str__ae__ng.tecken = string.char
      str__ae__ng.dumpa = string.dump
      str__ae__ng.hitta = string.find
      str__ae__ng.formatera = string.format
      str__ae__ng.l__ae__ngd = string.len
      str__ae__ng.omv__ae__nd = string.reverse
      str__ae__ng.del = sub
      str__ae__ng.gemener = string.lower
      str__ae__ng.versaler = string.upper

      tabell = {}
      tabell.sammanfoga = table.concat
      tabell.infoga = table.insert
      tabell.ta_bort = table.remove
      tabell.sortera = table.sort

      matte = {}
      matte.abs = math.abs
      matte.acos = math.acos
      matte.asin = math.asin
      matte.atan = math.atan
      matte.atan2 = math.atan2
      matte.cos = math.cos
      matte.deg = math.deg
      matte.exp = math.exp
      matte.golv = math.floor
      matte.tak = math.ceil
      matte.log = math.log
      matte.log10 = math.log10
      matte.max = math.max
      matte.min = math.min
      matte.pi = math.pi
      matte.upph__oe__jt = math.pow
      matte.rad = math.rad
      matte.slumpa = math.random
      matte.slumpfr__oe__ = math.randomseed
      matte.sin = math.sin
      matte.rot = math.sqrt
      matte.tan = math.tan

      io.st__ae__ng = io.close
      io.spola = io.flush
      io.rader = io.lines
      io.__oe__ppna = io.open
      io.l__ae__s = io.read
      io.typ = io.type

      os.klocka = os.clock
      os.datum = os.date
      os.tidsskillnad = os.difftime 
      os.utf__oe__r = os.execute
      os.avsluta = os.exit
      os.h__ae__mta_milj__oe__var = os.getenv
      os.ta_bort = os.remove
      os.byt_namn = os.rename
      os.tid = os.time
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