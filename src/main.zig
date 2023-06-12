const sol = @import("sol");

export fn entrypoint(_: [*]u8) callconv(.C) u64 {
    sol.print("Hello zig!", .{});
    return 0;
}
