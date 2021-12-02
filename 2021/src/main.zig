const std = @import("std");
const mem = std.mem;

pub fn day01_1(input: []const u8) !u32 {
    var iter = mem.split(input, "\n");

    var last_n: u32 = 0;
    var num_n: u32 = 0;
    while (iter.next()) |n| {
        // skip last item(empty)
        if (n.len == 0) {
            break;
        }

        // parse as u32
        const new_n: u32 = try std.fmt.parseUnsigned(u32, n, 10);

        if (new_n > last_n) {
            num_n += 1;
        }

        last_n = new_n;
    }

    // return, ignoring first result
    return num_n - 1;
}

test "day01 problem 1" {
    const input = @embedFile("../input/01.txt");

    const ret = try day01_1(input);

    try std.testing.expect(ret == 1559);
}
