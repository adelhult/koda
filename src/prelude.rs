pub fn get_prelude() -> &'static str {
    r#"
        par = pairs
        ipar = ipairs
        gemener = string.lower
        versaler = string.upper
        slumpa = math.random
        function t__ae__rning() return math.random(6) end
        tillstr__ae__ng = tostring(n)
        tillnummer = tonumber
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