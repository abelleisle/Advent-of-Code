ferry = {
    pos = {x = 0, y = 0},
    delta = {x = 1, y = 0},
    angle = 0
}

for line in io.lines("../input.txt") do
    if type(line) ~= nil then
        ins = line:sub(1,1)
        cnt = tonumber(line:sub(2))
        print('Dir: '.. ins .. ' Cnt: '.. cnt)

        if     ins == "N" then
            ferry.pos.y = ferry.pos.y + cnt
        elseif ins == "E" then
            ferry.pos.x = ferry.pos.x + cnt
        elseif ins == "S" then
            ferry.pos.y = ferry.pos.y - cnt
        elseif ins == "W" then
            ferry.pos.x = ferry.pos.x - cnt
        elseif ins == "R" or ins == "L" then
            if     ins == "R" then
                ferry.angle = ferry.angle - cnt
            elseif ins == "L" then
                ferry.angle = ferry.angle + cnt
            end
            ferry.delta.x = math.cos(ferry.angle * (math.pi/180))
            ferry.delta.y = math.sin(ferry.angle * (math.pi/180))
        elseif ins == "F" then
            ferry.pos.x = ferry.pos.x + (ferry.delta.x * cnt)
            ferry.pos.y = ferry.pos.y + (ferry.delta.y * cnt)
        end
    end
end

print('Final Position: '..ferry.pos.x..','..ferry.pos.y)
print('Final Manhattan: '..math.abs(ferry.pos.x)+math.abs(ferry.pos.y))
