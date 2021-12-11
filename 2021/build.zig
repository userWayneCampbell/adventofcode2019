const std = @import("std");

pub fn build(b: *std.build.Builder) void {
    var tests = b.addTest("src/01.zig");
    
    const test_step = b.step("test", "Run all tests");
    test_step.dependOn(&tests.step);
}
