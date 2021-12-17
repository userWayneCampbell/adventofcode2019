const std = @import("std");

pub fn build(b: *std.build.Builder) void {
    var test01 = b.addTest("src/01.zig");
    var test02 = b.addTest("src/02.zig");

    const test_step = b.step("test", "Run all tests");
    test_step.dependOn(&test01.step);
    test_step.dependOn(&test02.step);
}
