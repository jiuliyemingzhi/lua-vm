---
--- Generated by EmmyLua(https://github.com/EmmyLua)
--- Created by jiuli.
--- DateTime: 2023/7/11 01:38
---

local start_time = os.clock()

local result = 1
for i = 1, 100000000 do
    result = result +  1.1
end

local end_time = os.clock()
local elapsed_time = end_time - start_time

print("Elapsed Time: " .. elapsed_time .. " seconds " .. result)
