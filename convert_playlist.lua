local cjson = require "cjson"

local function readJson(path)
    local file = assert(io.open(path, "r"))
    local content = file:read("*all")
    file:close()
	return cjson.decode(content)
end

local function lookupId(entries, id)
	for i, video in ipairs(entries) do
		if video.id == id then
			return i
		end
	end
	return nil
end

local args = { ... }
-- Read our file
local json = readJson(args[1])
-- Check that we have an entries field
assert(json['entries'] ~= nil)

local indices = {}
local i = 1
for video in io.input():lines() do
	local id = video:reverse():sub(5, 15):reverse()
	local index = lookupId(json.entries, id);
	if index == nil then goto continue end
	indices[i] = {id = id, index = index}
	i = i + 1;
	::continue::
end
table.sort(indices, function(a,b)
	return a.index < b.index
end)

for k, _ in ipairs(indices) do 
	indices[k] = indices[k].id
end

io.write(cjson.encode(indices))
